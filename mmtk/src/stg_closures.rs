// TODO: add repr C to all the structs
// ------------ Closures.h ------------


// TODO: handle when profiling case
pub struct StgProfHeader {}

// ------------ Closure headers ------------
pub struct StgSMPThunkHeader {
    pub pad : StgWord
}

pub struct StgHeader {
    pub const info : *const StgInfoTable,
    pub prof_header : StgProfHeader,
}

pub struct StgThunkHeader {
    pub const info : *const StgInfoTable,
    pub prof_header : StgProfHeader,
    pub smp : StgSMPThunkHeader,
}

// ------------ Closure types ------------
pub struct StgClosure {
    pub header : StgHeader,
    // pub payload : *mut *mut StgClosure,
    // TODO: maybe don't use *mut *mut
}

// TODO: other ways to do this?
impl StgClosure {
    pub fn get_payload(&self, i: usize) -> *mut StgClosure {
        unimplemented!()
        let start_payload = (self as *const StgHeader).offset(1).cast::<*const *mut StgClosure>();
        start_payload.offset(i).deref()
    }
}

// Closure types: THUNK, THUNK_<X>_<Y>
pub struct StgThunk {
    pub header : StgThunkHeader,
    // pub payload : *mut *mut StgClosure,
}

// the same impl for getting payload?


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
    pub payload : *mut *mut StgClosure,
}

// Closure types: AP
pub struct AP {
    pub header : StgThunkHeader,
    pub arity : StgHalfWord,
    pub n_args : StgHalfWord,
    pub fun : *mut StgClosure,
    pub payload : *mut *mut StgClosure,
}

// Closure types: AP_STACK
pub struct StgAP_STACK {
    pub header : StgThunkHeader,
    pub size : StgWord,
    pub fun : *mut StgClosure,
    pub payload : *mut *mut StgClosure,
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
    pub const saved_info : *mut StgInfoTable,
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
pub struct StgArrBytes {
    pub header : StgHeader,
    pub bytes : StgWord, // number of bytes in payload
    pub pyaload : *mut StgWord, // Why is it StgWord here not StgClosure?
}

// Closure types: MUT_ARR_PTRS_CLEAN, MUT_ARR_PTRS_DIRTY,
// MUT_ARR_PTRS_FROZEN_DIRTY, MUT_ARR_PTRS_FROZEN_CLEAN, MUT_VAR_CLEAN,
// MUT_VAR_DIRTY
pub struct StgMutArr {
    pub header : StgHeader,
    pub ptrs : StgWord,
    pub size : StgWord,
    pub payload : *mut *mut StgClosure,
}

// Closure types: SMALL_MUT_ARR_PTRS_CLEAN, SMALL_MUT_ARR_PTRS_DIRTY,
// SMALL_MUT_ARR_PTRS_FROZEN_DIRTY, SMALL_MUT_ARR_PTRS_FROZEN_CLEAN,
pub struct StgSmallMutArr {
    pub header : StgHeader,
    pub ptrs : StgWord,
    pub payload : *mut *mut StgClosure,
}

// Closure types: MUT_VAR_CLEAN, MUT_VAR_DIRTY
pub struct StgMutVar {
    pub header : StgHeader,
    pub var : *StgClosure,
}

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

// Closure types: UNDERFLOW_FRAME
pub struct StgUnderflowFrame {
    pub const info : *mut StgClosureInfo,
    pub next_chunk : *mut StgStack, // TODO
}

// Closure types: STOP_FRAME
pub struct StgStopFrame {
    pub header : StgHeader,
}

// Closure types: RET_FUN
pub struct StgRetFun {
    pub const info : *mut StgClosureInfo,
    pub size : StgWord,
    pub fun : *mut StgClosure,
    pub payload : *mut *mut StgClosure,
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
    pub link : *mut link,
}

// Closure type: CONSTR
pub struct StgCFinalizerList {
    pub header : StgHeader,
    pub link : *mut StgClosure,
    // TODO: void pointer
}

// Closure types: BCO
pub struct StgBCO {
    pub header : StgHeader,
    pub instrs : *mut StgArrBytes,
    pub literals : *mut StgArrBytes,
    pub ptrs : *mut StgMutArr,
    pub arity : *mut StgHalfWord,
    pub size : *mut StgHalfWord,
    pub bitmap : StgLargeBitmap, // large bitmap = StgWord[] ?
}

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

// which closure type?
pub struct StgMVarTSOQueue {
    pub header : StgHeader,
    pub link : *mut StgMVarTSOQueue,
    pub tso : *mut StgTSO, // TODO
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
    pub num_updates : *mut StgInt,
}

pub struct TRecEntry {
    pub tvar : *mut StgTVar,
    pub expected_value : *mut StgClosure,
    pub new_value : *mut StgClosure,
}


static TREC_CHUNK_NUM_ENTRIES: i32 = 5;

pub struct StgTRecChunk {
    pub header : StgHeader,
    pub prev_chunk : *mut StgTRecChunk,
    pub next_entry_idx : StgWord,
    pub entries : [TRecEntry; TREC_CHUNK_NUM_ENTRIES], // unsure of using Box here
}

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
    pub link : *mut MessageThrowTo,
    pub source : *mut StgTSO,
    pub target : *mut StgTSO,
    pub exception : *mut StgClosure,
}

pub struct MessageBlackHole {
    pub header : StgHeader,
    pub link : *mut MessageThrowTo,
    pub tso : *mut StgTSO,
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
    pub self : *mut StgCompactNFDataBlock,
    pub owner : *mut StgCompactNFData,
    pub next : *mut StgCompactNFDataBlock,
}

pub struct StgCompactNFData {
    pub header : StgHeader,
    pub totalW : StgWord,
    pub autoBlockW : StgWord,
    pub hp : StgPtr,
    pub hpLim : StgPtr,
    pub nursery : *mut StgCompactNFDataBlock,
    pub last : *mut StgCompactNFDataBlock,
    pub hash : *mut Hashtable, // TODO
    pub result : *mut StgClosure,
    pub link : *mut StgCompactNFData,
}