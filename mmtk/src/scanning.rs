use crate::DummyVM;
use mmtk::scheduler::*;
use mmtk::util::opaque_pointer::*;
use mmtk::vm::{EdgeVisitor, Scanning};
use mmtk::util::{Address, ObjectReference};
use mmtk::Mutator;

use super::types::*;
use super::stg_closures::*;
use super::stg_info_table::*;
use std::cmp::min;
use std::mem::size_of;

pub struct VMScanning {}

fn scan_small_bitmap<EV: EdgeVisitor>(
    _tls: VMWorkerThread,
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

fn scan_large_bitmap<EV: EdgeVisitor>(
    _tls: VMWorkerThread,
    payload : &ClosurePayload,
    large_bitmap : &StgLargeBitmap,
    ev: &mut EV,
)
{
    let size_bits = large_bitmap.size;
    let mut b : usize = 0;
    let mut i : usize = 0;
    while i < size_bits {
        let mut bitmap = unsafe {*(large_bitmap.bitmap).get_w(b)};
        // word_len is the size is min(wordsize, (size_w - i) bits)
        let word_len = min(size_bits - i, 8*size_of::<StgWord>());
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


fn scan_stack<EV: EdgeVisitor>(
    _tls: VMWorkerThread,
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
                scan_small_bitmap(_tls, payload, bitmap, ev);
                scan_srt(_tls, &frame.header.info_table, ev);
            }
            RET_BIG(frame, bitmap_ref) => {
                let payload : &'static ClosurePayload = &frame.payload;
                scan_large_bitmap(_tls, payload, bitmap_ref, ev);
                scan_srt(_tls, &frame.header.info_table, ev);
            }
            RET_FUN_SMALL(frame, bitmap) => {
                ev.visit_edge(frame.fun.to_address());
                let payload : &'static ClosurePayload = &frame.payload;
                scan_small_bitmap(_tls, payload, bitmap, ev);
                scan_srt(_tls, &frame.info_table, ev);
            }
            RET_FUN_LARGE(frame, bitmap) => {
                ev.visit_edge(frame.fun.to_address());
                let payload : &'static ClosurePayload = &frame.payload;
                scan_large_bitmap(_tls, payload, bitmap, ev);
                scan_srt(_tls, &frame.info_table, ev);
            }
            _ => panic!("Unexpected stackframe type {:?}", stackframe)
       }
   }
}

fn scan_srt<EV: EdgeVisitor>(
    _tls: VMWorkerThread,
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

// fn scan_PAP_payload<EV: EdgeVisitor>(
//     _tls: VMWorkerThread,
//     fun_info: &StgFunInfoTable,
//     payload : &ClosurePayload,
//     size : StgWord,
//     ev: &mut EV,
// )
// {
//     use StgFunType::*;
//     match fun_info.f.fun_type {
//         ARG_GEN => {
//             let small_bitmap : StgSmallBitmap = fun_info.f.bitmap.small_bitmap;
//         }
//     }
// }

impl Scanning<DummyVM> for VMScanning {
    fn scan_thread_roots<W: ProcessEdgesWork<VM = DummyVM>>() {
        unimplemented!()
    }
    fn scan_thread_root<W: ProcessEdgesWork<VM = DummyVM>>(
        _mutator: &'static mut Mutator<DummyVM>,
        _tls: VMWorkerThread,
    ) {
        unimplemented!()
    }
    fn scan_vm_specific_roots<W: ProcessEdgesWork<VM = DummyVM>>() {
        unimplemented!()
    }

    /// Delegated scanning of a object, visiting each pointer field
    /// encountered.
    ///
    /// Arguments:
    /// * `tls`: The VM-specific thread-local storage for the current worker.
    /// * `object`: The object to be scanned.
    /// * `edge_visitor`: Called back for each edge.
    fn scan_object<EV: EdgeVisitor>(
        _tls: VMWorkerThread,
        obj: ObjectReference,
        ev: &mut EV,
    ) {
        unsafe {
            let closure_ref = TaggedClosureRef::from_object_reference(obj);            
            let itbl: &'static StgInfoTable = closure_ref.get_info_table();
            match closure_ref.to_closure() {
                Closure::MVar(mvar) => {
                    // let visit = |tagged_ref: &mut TaggedClosureRef| {
                    //     ev.visit_edge(tagged_ref.to_address())
                    // };

                    ev.visit_edge(Address::from_ptr(mvar.head));
                    ev.visit_edge(Address::from_ptr(mvar.tail));
                    ev.visit_edge(mvar.value.to_address());
                }
                Closure::TVar(tvar) => {
                    ev.visit_edge(tvar.current_value.to_address());
                    ev.visit_edge(Address::from_ptr(tvar.first_watch_queue_entry));
                }
                Closure::Thunk(thunk) => {
                    let n_ptrs : u32 = (&*itbl).layout.payload.ptrs;
                    for n in 0..n_ptrs {
                        let edge = thunk.payload.get(n as usize);
                        ev.visit_edge(edge.to_address());
                    }
                }
                Closure::Constr(closr) => {
                    let n_ptrs : u32 = (&*itbl).layout.payload.ptrs;
                    for n in 0..n_ptrs {
                        let edge = closr.payload.get(n as usize);
                        ev.visit_edge(edge.to_address());
                    }
                }
                Closure::Weak(weak) => {
                    ev.visit_edge(weak.value.to_address());
                    ev.visit_edge(weak.key.to_address());
                    ev.visit_edge(weak.finalizer.to_address());
                    ev.visit_edge(weak.cfinalizers.to_address());
                }
                Closure::BlockingQueue(bq) => {
                    ev.visit_edge(Address::from_ptr(bq.owner));
                    ev.visit_edge(Address::from_ptr(bq.queue));
                    ev.visit_edge(Address::from_ptr(bq.link));
                }
                Closure::ThunkSelector(selector) => {
                    ev.visit_edge(selector.selectee.to_address());
                }
                Closure::ApStack(fun) => {
                    ev.visit_edge(fun.fun.to_address());
                    scan_stack(_tls, fun.iter(), ev);
                }
                // TODO: FUN
                // TODO: PAP and AP using same struct?
                Closure::PartialAppliedFun(fun) => {
                    ev.visit_edge(fun.fun.to_address());
                    // TODO: scavenge_PAP_payload
                }
                Closure::AppliedFun(fun) => {
                    ev.visit_edge(fun.fun.to_address());
                    // TODO: scavenge_AP (just call scavenge_PAP_payload ?)
                }
                Closure::ArrBytes(_) => { return; }
                Closure::ArrMutPtr(array) => {
                    // TODO: scavenge_mut_arr_ptrs
                    // use scavenge_mut_arr_ptrs
                    // TODO: use the optimised version later for card marking
                    // should mark the bits : mark everything to 0 
                    // (bool: whether there's an inter generation pointer (old to young))
                    let n_ptrs = array.ptrs;
                    for n in 0..n_ptrs {
                        let edge = array.payload.get(n as usize);
                        ev.visit_edge(edge.to_address());
                    }
                }
                Closure::ArrMutPtrSmall(array) => {
                    let n_ptrs = array.ptrs;
                    for n in 0..n_ptrs {
                        let edge = array.payload.get(n);
                        ev.visit_edge(edge.to_address());
                    }
                }
                Closure::TSO(tso) => {
                    // if tso.bound != null {
                    //     ev.visit_edge(Address::from_ptr((&*tso.bound).tso));
                    // }
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
                Closure::Stack(stack) => {
                    scan_stack(_tls, stack.iter(), ev);
                }
                Closure::TRecChunk(trec_chunk) => {
                    ev.visit_edge(Address::from_ptr(trec_chunk.prev_chunk));

                    let n_ptrs = trec_chunk.next_entry_idx;
                    for n in 0..n_ptrs {
                        let trec_entry = &trec_chunk.entries[n];
                        ev.visit_edge(Address::from_ptr(trec_entry.tvar));
                        ev.visit_edge(trec_entry.expected_value.to_address());
                        ev.visit_edge(trec_entry.new_value.to_address());
                    }
                }
                Closure::Indirect(ind) => {
                    ev.visit_edge(Address::from_ptr(ind.indirectee.to_ptr()));
                }
                Closure::IndirectStatic(ind) => {
                    ev.visit_edge(Address::from_ptr(ind.indirectee.to_ptr()));
                }
                // TODO: scavenge_compact for COMPACT_NFDATA?
                _ => panic!("scavenge_one: strange object type={:?}, address={:?}", 
                            (&*itbl).type_, itbl)                
            }
            
            
        }

    }


    fn notify_initial_thread_scan_complete(_partial_scan: bool, _tls: VMWorkerThread) {
        unimplemented!()
    }
    fn supports_return_barrier() -> bool {
        unimplemented!()
    }
    fn prepare_for_roots_re_scanning() {
        unimplemented!()
    }
}