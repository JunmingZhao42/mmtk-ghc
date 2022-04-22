// TODO: add repr C to all the structs
// ------------ Closures.h ------------


// TODO: handle when profiling case
pub struct StgProfHeader {}

// ------------ Closure headers ------------
pub struct StgSMPThunkHeader {
    pub pad : StgWord
}

#[repr(C)]
struct StgInfoTableRef (*const StgInfoTable);

impl StgInfoTableRef {
    pub fn get_info_table(&self) -> &'static StgInfoTable {
        // some info table not always valid
        // load and unload codes make info table invalid
        if cfg!(tables_next_to_code) {
            self.0.offset(-1)
        } else {
            self.0
        }
    }
}

pub struct StgHeader {
    pub info_table: StgInfoTableRef,
    pub prof_header : StgProfHeader,
}

pub struct StgThunkHeader {
    pub info_table : StgInfoTableRef,
    pub prof_header : StgProfHeader,
    pub smp : StgSMPThunkHeader,
}

// ------------ payload ------------
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
pub struct StgClosure {
    pub header  : StgHeader,
    pub payload : ClosurePayload,
}

// Closure types: THUNK, THUNK_<X>_<Y>
pub struct StgThunk {
    pub header  : StgThunkHeader,
    pub payload : ClosurePayload,
}

// Closure types: THUNK_SELECTOR
pub struct StgSelector {
    pub header : StgThunkHeader,
    pub selectee : *mut StgClosure,
}

// Closure types: PAP
pub struct StgPAP {
    pub header : StgHeader,
    pub arity : StgHalfWord,
    pub n_args : StgHalfWord,
    pub fun : *mut StgClosure,
    pub payload : ClosurePayload,
}

// Closure types: AP
pub struct StgAP {
    pub header : StgThunkHeader,
    pub arity : StgHalfWord,
    pub n_args : StgHalfWord,
    pub fun : *mut StgClosure,
    pub payload : ClosurePayload,
}

// Closure types: AP_STACK
pub struct StgAP_STACK {
    pub header : StgThunkHeader,
    pub size : StgWord,
    pub fun : *mut StgClosure,
    pub payload : ClosurePayload,
}

// Closure types: IND
pub struct StgInd {
    pub header : StgHeader,
    pub indirectee : *mut StgClosure,
}

// Closure types: IND_STATIC
pub struct StgIndStatic {
    pub header : StgHeader,
    pub indirectee : *mut StgClosure,
    pub static_link : *mut StgClosure,
    pub saved_info_table : StgInfoTableRef,
}

// Closure types: BLOCKING_QUEUE
pub struct StgBlockingQueue {
    pub header : StgHeader,
    pub link : *mut StgBlockingQueue,
    pub bh : *mut StgClosure,
    pub owner : *mut StgTSO, // TODO: StgTSO
    pub queue: *mut MessageBlackHole,
}

// Closure types: ARR_WORDS
// an array of bytes -- a buffer of memory
pub struct StgArrBytes {
    pub header : StgHeader,
    pub bytes : StgWord, // number of bytes in payload
    // pub payload : *mut StgWord, // Why is it StgWord here not StgClosure?
}

// Closure types: MUT_ARR_PTRS_CLEAN, MUT_ARR_PTRS_DIRTY,
// MUT_ARR_PTRS_FROZEN_DIRTY, MUT_ARR_PTRS_FROZEN_CLEAN, MUT_VAR_CLEAN,
// MUT_VAR_DIRTY
pub struct StgMutArrPtrs {
    pub header : StgHeader,
    pub ptrs : StgWord,
    pub size : StgWord,
    pub payload : ClosurePayload,
}

// Closure types: SMALL_MUT_ARR_PTRS_CLEAN, SMALL_MUT_ARR_PTRS_DIRTY,
// SMALL_MUT_ARR_PTRS_FROZEN_DIRTY, SMALL_MUT_ARR_PTRS_FROZEN_CLEAN,
pub struct StgSmallMutArrPtrs {
    pub header : StgHeader,
    pub ptrs : StgWord,
    pub payload : ClosurePayload,
}

// Closure types: MUT_VAR_CLEAN, MUT_VAR_DIRTY
pub struct StgMutVar {
    pub header : StgHeader,
    pub var : *mut StgClosure,
}

// ------ stack frames -----------


// Closure types: UPDATE_FRAME
pub struct StgUpdateFrame {
    pub header : StgHeader,
    pub updatee : *mut StgClosure,
}

// Closure types: CATCH_FRAME
pub struct StgCatchFrame {
    pub header : StgHeader,
    pub exceptions_blocked : StgWord,
    pub handler : *mut StgClosure,
}

pub struct StgStackPayload {}

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
pub struct StgUnderflowFrame {
    pub info_table : StgInfoTableRef,
    pub next_chunk : *mut StgStack,
}

// Closure types: STOP_FRAME
pub struct StgStopFrame {
    pub header : StgHeader,
}

// Closure types: RET_FUN
pub struct StgRetFun {
    pub info_table : StgInfoTableRef,
    pub size : StgWord,
    pub fun : *mut StgClosure,
    pub payload : ClosurePayload,
}

// Closure type: CONSTR_0_1
pub struct StgIntCharlikeClosure {
    pub header : StgHeader,
    pub data : StgWord,
}

// Stable name, StableName# v
pub struct StgStableName {
    pub header : StgHeader,
    pub sn : StgWord,
}

// Closure types: WEAK
pub struct StgWeak {
    pub header : StgHeader,
    pub cfinalizers : *mut StgClosure,
    pub key : *mut StgClosure,
    pub value : *mut StgClosure,
    pub finalizer : *mut StgClosure,
    pub link : *mut StgWeak,
}


union FinalizerFn {
    pub without_env: *const extern "C" fn(*mut u8),
      // ^ (ptr)
    pub with_env: *const extern "C" fn(*mut u8, *mut u8)
      // ^ (eptr, ptr)
}

// Closure type: CONSTR
struct StgCFinalizerList {
    header: StgHeader,
    link: *mut StgClosure,
    finalize: FinalizerFn,
    ptr: *mut u8,
    eptr: *mut u8,
    flag: StgWord,
}

impl StgCFinalizer {
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
pub struct StgMVarTSOQueue {
    pub header : StgHeader,
    pub link : *mut StgMVarTSOQueue,
    pub tso : *mut StgTSO, // TODO: define TSO
}

// Closure types: MVAR_CLEAN, MVAR_DIRTY
pub struct StgMVar {
    pub header : StgHeader,
    pub head : *mut StgMVarTSOQueue,
    pub tail : *mut StgMVarTSOQueue,
    pub value : *mut StgClosure,
}

pub struct StgTVarWatchQueue {
    pub header : StgHeader,
    pub closure : *mut StgTSO,
    pub next_queue_entry : *mut StgTVarWatchQueue,
    pub prev_queue_entry : *mut StgTVarWatchQueue,
}

pub struct StgTVar {
    pub header : StgHeader,
    pub current_value : *mut StgClosure,
    pub first_watch_queue_entry : *mut StgTVarWatchQueue,
    pub num_updates : StgInt,
}

pub struct TRecEntry {
    pub tvar : *mut StgTVar,
    pub expected_value : *mut StgClosure,
    pub new_value : *mut StgClosure,
    // TODO: add num_updates when THREADED_RTS
}


const TREC_CHUNK_NUM_ENTRIES: i32 = 16;

// contains many TRec entries and link them together
pub struct StgTRecChunk {
    pub header : StgHeader,
    pub prev_chunk : *mut StgTRecChunk,
    pub next_entry_idx : StgWord,
    pub entries : [TRecEntry; TREC_CHUNK_NUM_ENTRIES], 
}

// maybe don't need this
pub enum TRecState {
    TREC_ACTIVE,        /* Transaction in progress, outcome undecided */
    TREC_CONDEMNED,     /* Transaction in progress, inconsistent / out of date reads */
    TREC_COMMITTED,     /* Transaction has committed, now updating tvars */
    TREC_ABORTED,       /* Transaction has aborted, now reverting tvars */
    TREC_WAITING,       /* Transaction currently waiting */
}

pub struct StgTRecHeader {
    pub header : StgHeader,
    pub enclosing_trec : *mut StgTRecHeader,
    pub current_chunk : *mut StgTRecChunk,
    pub state : TRecState,
}

pub struct StgAtomicallyFrame {
    pub header : StgHeader,
    pub code : *mut StgClosure,
    pub result : *mut StgClosure,
}

pub struct StgCatchSTMFrame {
    pub header : StgHeader,
    pub code : *mut StgClosure,
    pub handler : *mut StgClosure,
}

pub struct StgCatchRetryFrame {
    pub header : StgHeader,
    pub running_alt_code : StgWord,
    pub first_code : *mut StgClosure,
    pub alt_code : *mut StgClosure,
}


/* ----------------------------------------------------------------------------
   Messages
   ------------------------------------------------------------------------- */

pub struct Message {
    pub header : StgHeader,
    pub link : *mut Message,
}

pub struct MessageWakeup {
    pub header : StgHeader,
    pub link : *mut Message,
    pub tso : *mut StgTSO,
}

pub struct MessageThrowTo {
    pub header : StgHeader,
    pub link : *mut MessageThrowTo, // should be just Message ?
    pub source : *mut StgTSO,
    pub target : *mut StgTSO,
    pub exception : *mut StgClosure,
}

pub struct MessageBlackHole {
    pub header : StgHeader,
    pub link : *mut MessageBlackHole, // should be just Message ?
    pub tso : *mut StgTSO,
    pub bh : *mut StgClosure,
}

pub struct MessageCloneStack {
    pub header : StgHeader,
    pub link : *mut Message,
    pub result : *mut StgMVar,
    pub tso : *mut StgTSO,
}


/* ----------------------------------------------------------------------------
   Compact Regions
   ------------------------------------------------------------------------- */

pub struct StgCompactNFDataBlock {
    pub self_ : *mut StgCompactNFDataBlock,
    pub owner : *mut StgCompactNFData,
    pub next : *mut StgCompactNFDataBlock,
}

pub struct Hashtable {}

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

// TODO: test out