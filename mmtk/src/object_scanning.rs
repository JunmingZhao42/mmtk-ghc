// use mmtk::util::opaque_pointer::*;
use mmtk::vm::{EdgeVisitor};
use mmtk::util::{Address};

use super::types::*;
use super::stg_closures::*;
use super::stg_info_table::*;

use std::cmp::min;
use std::mem::size_of;

pub fn scan_closure_payload<EV: EdgeVisitor>(
    // _tls: VMWorkerThread,
    payload : &ClosurePayload,
    n_ptrs : u32,
    ev: &mut EV,
)
{
    for n in 0..n_ptrs {
        let edge = payload.get(n as usize);
        ev.visit_edge(edge.to_address());
    }
}

#[allow(non_snake_case)]
pub fn scan_TSO<EV: EdgeVisitor>(
    // _tls: VMWorkerThread,
    tso : &StgTSO,
    ev: &mut EV,
)
{
    // update the pointer from the InCall
    if tso.bound.is_null() {
        unsafe {ev.visit_edge(Address::from_ptr((&*tso.bound).tso));}
    }

    ev.visit_edge(Address::from_ptr(tso.blocked_exceptions));
    ev.visit_edge(Address::from_ptr(tso.blocking_queue));
    ev.visit_edge(Address::from_ptr(tso.trec));
    ev.visit_edge(Address::from_ptr(tso.stackobj));
    ev.visit_edge(Address::from_ptr(tso.link));

    if tso.why_blocked == StgTSOBlocked::BLOCKED_ON_MVAR
    || tso.why_blocked == StgTSOBlocked::BLOCKED_ON_MVAR_READ
    || tso.why_blocked == StgTSOBlocked::BLOCKED_ON_BLACK_HOLE
    || tso.why_blocked == StgTSOBlocked::BLOCKED_ON_MSG_THROW_TO
    || tso.why_blocked == StgTSOBlocked::NOT_BLOCKED {
        ev.visit_edge(tso.block_info.closure.to_address());
    }

    ev.visit_edge(Address::from_ptr(tso.tso_link_prev));
    ev.visit_edge(Address::from_ptr(tso.tso_link_next));
}

#[allow(non_snake_case)]
pub fn scan_PAP_payload<EV: EdgeVisitor>(
    // _tls: VMWorkerThread,
    fun_info: &StgFunInfoTable,
    payload : &ClosurePayload,
    size : usize,
    ev: &mut EV,
)
{
    use StgFunType::*;
    debug_assert_ne!(fun_info.i.type_, StgClosureType::PAP);

    match fun_info.f.fun_type {
        ARG_GEN => unsafe {
            let small_bitmap : StgSmallBitmap = fun_info.f.bitmap.small_bitmap;
            scan_small_bitmap( payload, small_bitmap, ev);
        }
        ARG_GEN_BIG => unsafe {
            let large_bitmap : &StgLargeBitmap = 
                &*(fun_info.f.bitmap.large_bitmap_ref.deref(fun_info));
            scan_large_bitmap( payload, large_bitmap, size, ev);
        }
        // TODO: handle ARG_BCO case
        _ => {
            let small_bitmap = StgFunType::get_small_bitmap(&fun_info.f.fun_type);
            scan_small_bitmap( payload, small_bitmap, ev);
        }
    }
}

static MUT_ARR_PTRS_CARD_BITS : usize = 7;


/**
 * Scan mutable arrays of pointers
 */
pub unsafe fn scan_mut_arr_ptrs<EV: EdgeVisitor>(
    // _tls: VMWorkerThread,
    array : &StgMutArrPtrs,
    ev: &mut EV,
)
{
    // number of cards in the array
    let n_cards : StgWord = (array.n_ptrs + (1 << MUT_ARR_PTRS_CARD_BITS) - 1) 
                            >> MUT_ARR_PTRS_CARD_BITS;

    // scan card 0..n-1
    for m in 0..n_cards-1 {
        // m-th card, iterate through 2^MUT_ARR_PTRS_CARD_BITS many elements
        for p in m*(1<<MUT_ARR_PTRS_CARD_BITS) .. (m+1)*(1<<MUT_ARR_PTRS_CARD_BITS) {
            let edge = array.payload.get(p);
            ev.visit_edge(edge.to_address());

            // mark m-th card to 0
            let m_card_address : *const StgWord8 = (array.payload.get(array.n_ptrs).to_ptr() 
                                                    as usize + m) as *const StgWord8;
            let mut _m_card_mark = &*m_card_address;
            _m_card_mark = &0;
        }
    }

    // scan the last card (no need to scan entirely)
    for p in (n_cards-1)*(1<<MUT_ARR_PTRS_CARD_BITS) .. array.n_ptrs {
        let edge = array.payload.get(p);
        ev.visit_edge(edge.to_address());

        // mark m-th card to 0
        let m_card_address : *const StgWord8 = (array.payload.get(array.n_ptrs).to_ptr() 
                                                as usize + (n_cards-1)) as *const StgWord8;
        let mut _m_card_mark = &*m_card_address;
        _m_card_mark = &0;
    }

    // TODO: use the optimised version later for card marking
    // (bool: whether there's an inter generation pointer (old to young))
}

pub fn scan_small_bitmap<EV: EdgeVisitor>(
    // _tls: VMWorkerThread,
    payload : &ClosurePayload,
    small_bitmap : StgSmallBitmap,
    ev: &mut EV,
)
{
    let size = small_bitmap.size();
    let mut bitmap = small_bitmap.bits();

    for i in 0..size {
        if (bitmap & 1) == 0 {
            ev.visit_edge(payload.get(i).to_address());
        }
        bitmap = bitmap >> 1;
    }
}

pub fn scan_large_bitmap<EV: EdgeVisitor>(
    // _tls: VMWorkerThread,
    payload : &ClosurePayload,
    large_bitmap : &StgLargeBitmap,
    size : usize,
    ev: &mut EV,
)
{
    // Bitmap may have more bits than `size` when scavenging PAP payloads
    // PAP n_args < fun.bitmap.size
    // AP n_args = fun.bitmap.size
    debug_assert!(size <= large_bitmap.size);

    let mut b : usize = 0;
    let mut i : usize = 0;
    while i < size {
        let mut bitmap = unsafe {*(large_bitmap.bitmap).get_w(b)};
        // word_len is the size is min(wordsize, (size_w - i) bits)
        let word_len = min(size - i, 8*size_of::<StgWord>());
        i += word_len;
        for j in 0..word_len {
            if (bitmap & 1) == 0 {
                ev.visit_edge(payload.get(j).to_address());
            }
            bitmap = bitmap >> 1;
        }
        b += 1;
    }
}


pub fn scan_stack<EV: EdgeVisitor>(
    // _tls: VMWorkerThread,
    stack : StackIterator,
    ev: &mut EV,
)
{
    for stackframe in stack {
        use StackFrame::*;
        match stackframe {
            UPD_FRAME(frame) => {
                ev.visit_edge(frame.updatee.to_address());
            }
            RET_SMALL(frame, bitmap) => {
                let payload : &'static ClosurePayload = &frame.payload;
                scan_small_bitmap( payload, bitmap, ev);
                scan_srt(&frame.header.info_table, ev);
            }
            RET_BIG(frame, bitmap_ref) => {
                let payload : &'static ClosurePayload = &frame.payload;
                let size : usize = bitmap_ref.size;
                scan_large_bitmap( payload, bitmap_ref, size, ev);
                scan_srt(&frame.header.info_table, ev);
            }
            RET_FUN_SMALL(frame, bitmap) => {
                ev.visit_edge(frame.fun.to_address());
                let payload : &'static ClosurePayload = &frame.payload;
                scan_small_bitmap(payload, bitmap, ev);
                scan_srt(&frame.info_table, ev);
            }
            RET_FUN_LARGE(frame, bitmap_ref) => {
                ev.visit_edge(frame.fun.to_address());
                let payload : &'static ClosurePayload = &frame.payload;
                let size : usize = bitmap_ref.size;
                scan_large_bitmap(payload, bitmap_ref, size, ev);
                scan_srt(&frame.info_table, ev);
            }
            _ => panic!("Unexpected stackframe type {:?}", stackframe)
       }
   }
}

pub fn scan_srt<EV: EdgeVisitor>(
   // _tls: VMWorkerThread,
    ret_info_table : &StgRetInfoTable,
    ev: &mut EV,
)
{
    // TODO: only for major gc
    // TODO: non USE_INLINE_SRT_FIELD
    match ret_info_table.get_srt() {
        None => (),
        Some(srt) => {
            ev.visit_edge(Address::from_ptr(srt));
        }
    }
}

pub fn scan_srt_thunk<EV: EdgeVisitor>(
    // _tls: VMWorkerThread,
    thunk_info_table : &StgThunkInfoTable,
    ev: &mut EV,
)
{
    // TODO: only for major gc
    // TODO: non USE_INLINE_SRT_FIELD
    match thunk_info_table.get_srt() {
        None => (),
        Some(srt) => {
            ev.visit_edge(Address::from_ptr(srt));
        }
    }
}

pub fn scan_srt_fun<EV: EdgeVisitor>(
    // _tls: VMWorkerThread,
    fun_info_table : &StgFunInfoTable,
    ev: &mut EV,
)
{
    // TODO: only for major gc
    // TODO: non USE_INLINE_SRT_FIELD
    match fun_info_table.get_srt() {
        None => (),
        Some(srt) => {
            ev.visit_edge(Address::from_ptr(srt));
        }
    }
}
