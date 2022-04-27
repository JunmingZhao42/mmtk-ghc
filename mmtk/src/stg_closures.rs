use crate::DummyVM;
use super::types::*;
use super::stg_info_table::*;
// ------------ Closures.h ------------


// TODO: handle when profiling case
#[repr(C)]
pub struct StgProfHeader {}

// ------------ Closure headers ------------
#[repr(C)]
pub struct StgSMPThunkHeader {
    pub pad : StgWord
}

// TODO: make correspoinding comments
// safe way to dereference
#[repr(C)]
struct StgInfoTableRef (*const StgInfoTable);

impl StgInfoTableRef {
    pub fn get_info_table(&self) -> *const StgInfoTable {
        // some info table not always valid
        // load and unload codes make info table invalid
        if cfg!(tables_next_to_code) {
            self.0.offset(-1)
        } else {
            self.0
        }
    }
}

#[repr(C)]
pub struct StgHeader {
    pub info_table: StgInfoTableRef,
    pub prof_header : StgProfHeader,
}

#[repr(C)]
pub struct StgThunkHeader {
    pub info_table : StgInfoTableRef,
    pub prof_header : StgProfHeader,
    pub smp : StgSMPThunkHeader,
}

// ------------ payload ------------
#[repr(C)]
pub struct ClosurePayload {}

// TODO: check other instances of indexing in payload
impl ClosurePayload {
    pub fn get(&self, i: usize) -> *mut StgClosure {
        unsafe {
            let ptr: *const ClosurePayload = &*self;
            let payload: *const *mut StgClosure = ptr.cast();
            *payload.offset(i as isize)
        }
    }
}

// ------------ Closure types ------------
#[repr(C)]
pub struct StgClosure {
    pub header  : StgHeader,
    pub payload : ClosurePayload,
}

// Closure types: THUNK, THUNK_<X>_<Y>
#[repr(C)]
pub struct StgThunk {
    pub header  : StgThunkHeader,
    pub payload : ClosurePayload,
}

// Closure types: THUNK_SELECTOR
#[repr(C)]
pub struct StgSelector {
    pub header : StgThunkHeader,
    pub selectee : *mut StgClosure,
}

// Closure types: PAP
#[repr(C)]
pub struct StgPAP {
    pub header : StgHeader,
    pub arity : StgHalfWord,
    pub n_args : StgHalfWord,
    pub fun : *mut StgClosure,
    pub payload : ClosurePayload,
}

// Closure types: AP
#[repr(C)]
pub struct StgAP {
    pub header : StgThunkHeader,
    pub arity : StgHalfWord,
    pub n_args : StgHalfWord,
    pub fun : *mut StgClosure,
    pub payload : ClosurePayload,
}

// Closure types: AP_STACK
#[repr(C)]
pub struct StgAP_STACK {
    pub header : StgThunkHeader,
    pub size : StgWord,
    pub fun : *mut StgClosure,
    pub payload : ClosurePayload,
}

// Closure types: IND
#[repr(C)]
pub struct StgInd {
    pub header : StgHeader,
    pub indirectee : *mut StgClosure,
}

// Closure types: IND_STATIC
#[repr(C)]
pub struct StgIndStatic {
    pub header : StgHeader,
    pub indirectee : *mut StgClosure,
    pub static_link : *mut StgClosure,
    pub saved_info_table : StgInfoTableRef,
}

// Closure types: BLOCKING_QUEUE
#[repr(C)]
pub struct StgBlockingQueue {
    pub header : StgHeader,
    pub link : *mut StgBlockingQueue,
    pub bh : *mut StgClosure,
    pub owner : *mut StgTSO, // TODO: StgTSO
    pub queue : *mut MessageBlackHole,
}

#[repr(C)]
pub struct StgTSO {
    pub header : StgHeader,
    pub link : *mut StgTSO,
    pub global_link : *mut StgTSO,
    pub tso_link_prev : *mut StgTSO,
    pub tso_link_next : *mut StgTSO,
    pub stackobj : *mut StgStack,
    pub what_next : StgTSONext, // in types.rs
    pub why_blocked : StgTSOBlocked,  // in types.rs
    pub flags : StgTSOFlag,   // in types.rs
    pub block_info : StgTSOBlockInfo,
    pub id : StgThreadID, 
    pub saved_errno : StgWord32,
    pub dirty : StgWord32,
    pub bound : *mut  InCall,
    pub cap : *mut Capability,
    pub trec : *mut StgTRecHeader,
    pub blocked_exceptions : *mut MessageThrowTo,
    pub blocking_queue : *mut StgBlockingQueue,
    pub alloc_limit : StgInt64,
    pub tot_stack_size : StgWord32,

    // TODO: handle TICKY_TICKY, PROFILING, mingw32_HOST_OS
}

#[repr(C)]
pub struct StgThreadID(StgWord64);

// TODO: here are some dummy structs to complete fields in TSO
#[repr(C)]
pub struct InCall {}

#[repr(C)]
pub struct StgTSOBlockInfo{}

#[repr(C)]
pub struct Capability {}


// Closure types: ARR_WORDS
// an array of bytes -- a buffer of memory
#[repr(C)]
pub struct StgArrBytes {
    pub header : StgHeader,
    pub bytes : StgWord, // number of bytes in payload
    // pub payload : *mut StgWord, // Why is it StgWord here not StgClosure?
}

// Closure types: MUT_ARR_PTRS_CLEAN, MUT_ARR_PTRS_DIRTY,
// MUT_ARR_PTRS_FROZEN_DIRTY, MUT_ARR_PTRS_FROZEN_CLEAN, MUT_VAR_CLEAN,
// MUT_VAR_DIRTY
#[repr(C)]
pub struct StgMutArrPtrs {
    pub header : StgHeader,
    pub ptrs : StgWord,
    pub size : StgWord,
    pub payload : ClosurePayload,
}

// Closure types: SMALL_MUT_ARR_PTRS_CLEAN, SMALL_MUT_ARR_PTRS_DIRTY,
// SMALL_MUT_ARR_PTRS_FROZEN_DIRTY, SMALL_MUT_ARR_PTRS_FROZEN_CLEAN,
#[repr(C)]
pub struct StgSmallMutArrPtrs {
    pub header : StgHeader,
    pub ptrs : StgWord,
    pub payload : ClosurePayload,
}

// Closure types: MUT_VAR_CLEAN, MUT_VAR_DIRTY
#[repr(C)]
pub struct StgMutVar {
    pub header : StgHeader,
    pub var : *mut StgClosure,
}

// ------ stack frames -----------


// Closure types: UPDATE_FRAME
#[repr(C)]
pub struct StgUpdateFrame {
    pub header : StgHeader,
    pub updatee : *mut StgClosure,
}

// Closure types: CATCH_FRAME
#[repr(C)]
pub struct StgCatchFrame {
    pub header : StgHeader,
    pub exceptions_blocked : StgWord,
    pub handler : *mut StgClosure,
}

#[repr(C)]
pub struct StgStackPayload {}

#[repr(C)]
pub struct StgStack {
    pub header : StgHeader,
    pub stack_size : StgWord32,
    pub dirty : StgWord8,
    pub marking : StgWord8,
    pub sp : *mut StgWord,
    pub stack : StgStackPayload,
}

// impl walk through stack?

// Closure types: UNDERFLOW_FRAME
#[repr(C)]
pub struct StgUnderflowFrame {
    pub info_table : StgInfoTableRef,
    pub next_chunk : *mut StgStack,
}

// Closure types: STOP_FRAME
#[repr(C)]
pub struct StgStopFrame {
    pub header : StgHeader,
}

// Closure types: RET_FUN
#[repr(C)]
pub struct StgRetFun {
    pub info_table : StgInfoTableRef,
    pub size : StgWord,
    pub fun : *mut StgClosure,
    pub payload : ClosurePayload,
}

// Closure type: CONSTR_0_1
#[repr(C)]
pub struct StgIntCharlikeClosure {
    pub header : StgHeader,
    pub data : StgWord,
}

// Stable name, StableName# v
#[repr(C)]
pub struct StgStableName {
    pub header : StgHeader,
    pub sn : StgWord,
}

// Closure types: WEAK
#[repr(C)]
pub struct StgWeak {
    pub header : StgHeader,
    pub cfinalizers : *mut StgClosure,
    pub key : *mut StgClosure,
    pub value : *mut StgClosure,
    pub finalizer : *mut StgClosure,
    pub link : *mut StgWeak,
}


#[repr(C)]
union FinalizerFn {
    pub without_env: *const extern "C" fn(*mut u8),
      // ^ (ptr)
    pub with_env: *const extern "C" fn(*mut u8, *mut u8)
      // ^ (eptr, ptr)
}

// Closure type: CONSTR
#[repr(C)]
pub struct StgCFinalizerList {
    header: StgHeader,
    link: *mut StgClosure,
    finalize: FinalizerFn,
    ptr: *mut u8,
    eptr: *mut u8,
    flag: StgWord,
}

impl StgCFinalizerList {
    // example of how to use
    pub unsafe fn run(&self) {
        match self.flag {
            0 => (*self.finalize.without_env)(self.ptr),
            1 => (*self.finalize.with_env)(self.eptr, self.ptr),
            _ => panic!("oh no!")
        }
    }
}

// Closure types: BCO
#[repr(C)]
pub struct StgBCO {
    pub header : StgHeader,
    pub instrs : *mut StgArrBytes,
    pub literals : *mut StgArrBytes,
    pub ptrs : *mut StgMutArrPtrs,
    pub arity : StgHalfWord,
    pub size : StgHalfWord,
    pub bitmap : StgLargeBitmap, // TODO: large bitmap ? check
}

/*
TODO: have a look at BCO functions later
impl StgBCO {
    // TODO: inline functions of StgBCO
    #[inline(always)]
    pub fn BCO_BITMAP(&self) -> *mut StgLargeBitmap {
        unimplemented!()
    }

    #[inline(always)]
    pub fn BCO_BITMAP_SIZE(&self) -> StgWord {
        unimplemented!()
    }

    #[inline(always)]
    pub fn BCO_BITMAP_SIZE(&self) -> StgLargeBitmap {
        unimplemented!()
    }

    #[inline(always)]
    pub fn BCO_BITMAP_SIZEW(&self) -> StgWord {
        unimplemented!()
    }
}
*/

// which closure type?
#[repr(C)]
pub struct StgMVarTSOQueue {
    pub header : StgHeader,
    pub link : *mut StgMVarTSOQueue,
    pub tso : *mut StgTSO, // TODO: define TSO
}

// Closure types: MVAR_CLEAN, MVAR_DIRTY
#[repr(C)]
pub struct StgMVar {
    pub header : StgHeader,
    pub head : *mut StgMVarTSOQueue,
    pub tail : *mut StgMVarTSOQueue,
    pub value : *mut StgClosure,
}

#[repr(C)]
pub struct StgTVarWatchQueue {
    pub header : StgHeader,
    pub closure : *mut StgTSO,
    pub next_queue_entry : *mut StgTVarWatchQueue,
    pub prev_queue_entry : *mut StgTVarWatchQueue,
}

#[repr(C)]
pub struct StgTVar {
    pub header : StgHeader,
    pub current_value : *mut StgClosure,
    pub first_watch_queue_entry : *mut StgTVarWatchQueue,
    pub num_updates : StgInt,
}

#[repr(C)]
pub struct TRecEntry {
    pub tvar : *mut StgTVar,
    pub expected_value : *mut StgClosure,
    pub new_value : *mut StgClosure,
    // TODO: add num_updates when THREADED_RTS
}


const TREC_CHUNK_NUM_ENTRIES: usize = 16;

// contains many TRec entries and link them together
#[repr(C)]
pub struct StgTRecChunk {
    pub header : StgHeader,
    pub prev_chunk : *mut StgTRecChunk,
    pub next_entry_idx : StgWord,
    pub entries : [TRecEntry; TREC_CHUNK_NUM_ENTRIES], 
}

// maybe don't need this
pub enum TRecState {
    TrecActive,        /* Transaction in progress, outcome undecided */
    TrecCondemned,     /* Transaction in progress, inconsistent / out of date reads */
    TrecCommitted,     /* Transaction has committed, now updating tvars */
    TrecAborted,       /* Transaction has aborted, now reverting tvars */
    TrecWaiting,       /* Transaction currently waiting */
}

#[repr(C)]
pub struct StgTRecHeader {
    pub header : StgHeader,
    pub enclosing_trec : *mut StgTRecHeader,
    pub current_chunk : *mut StgTRecChunk,
    pub state : TRecState,
}

#[repr(C)]
pub struct StgAtomicallyFrame {
    pub header : StgHeader,
    pub code : *mut StgClosure,
    pub result : *mut StgClosure,
}

#[repr(C)]
pub struct StgCatchSTMFrame {
    pub header : StgHeader,
    pub code : *mut StgClosure,
    pub handler : *mut StgClosure,
}

#[repr(C)]
pub struct StgCatchRetryFrame {
    pub header : StgHeader,
    pub running_alt_code : StgWord,
    pub first_code : *mut StgClosure,
    pub alt_code : *mut StgClosure,
}


/* ----------------------------------------------------------------------------
   Messages
   ------------------------------------------------------------------------- */

#[repr(C)]
pub struct Message {
    pub header : StgHeader,
    pub link : *mut Message,
}

#[repr(C)]
pub struct MessageWakeup {
    pub header : StgHeader,
    pub link : *mut Message,
    pub tso : *mut StgTSO,
}

#[repr(C)]
pub struct MessageThrowTo {
    pub header : StgHeader,
    pub link : *mut MessageThrowTo, // should be just Message ?
    pub source : *mut StgTSO,
    pub target : *mut StgTSO,
    pub exception : *mut StgClosure,
}

#[repr(C)]
pub struct MessageBlackHole {
    pub header : StgHeader,
    pub link : *mut MessageBlackHole, // should be just Message ?
    pub tso : *mut StgTSO,
    pub bh : *mut StgClosure,
}

#[repr(C)]
pub struct MessageCloneStack {
    pub header : StgHeader,
    pub link : *mut Message,
    pub result : *mut StgMVar,
    pub tso : *mut StgTSO,
}


/* ----------------------------------------------------------------------------
   Compact Regions
   ------------------------------------------------------------------------- */
#[repr(C)]
pub struct StgCompactNFDataBlock {
    pub self_ : *mut StgCompactNFDataBlock,
    pub owner : *mut StgCompactNFData,
    pub next : *mut StgCompactNFDataBlock,
}

#[repr(C)]
pub struct Hashtable {}

#[repr(C)]
pub struct StgCompactNFData {
    pub header : StgHeader,
    pub totalW : StgWord,
    pub autoBlockW : StgWord,
    pub hp : StgPtr,
    pub hpLim : StgPtr,
    pub nursery : *mut StgCompactNFDataBlock,
    pub last : *mut StgCompactNFDataBlock,
    pub hash : *mut Hashtable, // TODO: define HashTable
    pub result : *mut StgClosure,
    pub link : *mut StgCompactNFData, // maybe need to rework compact normal form
}

// TODO: test with some typical haskell objects for object scanning