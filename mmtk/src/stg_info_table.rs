// ------------ InfoTables.h ------------

/* -----------------------------------------------------------------------------
   Profiling info
   -------------------------------------------------------------------------- */

pub struct StgProfInfo {
    
}


/* -----------------------------------------------------------------------------
   Closure flags
   -------------------------------------------------------------------------- */

#[repr(C)]
pub struct ClosureFlag (StgWord16),

impl ClosureFlag {
    // TODO: implement Flag related macro
    pub const HNF : StgWord16 = (1<<0),

    #[inline(always)]
    pub fn get_closure_flags(StgClosureType) -> StgWord16 {
        unimplemented!()
    }
    // TODO: implement by rts/ClosureFlags.c
}


// bitmaps
pub struct Bitmap (StgWord),

impl Bitmap {
    #[inline(always)]
    pub fn MK_SMALL_BITMAP(size, bit) -> Self {
        unimplemented!()
    }

    #[inline(always)]
    pub fn BITMAP_SIZE(&self) -> StgWord {
        unimplemented!()
    }

    #[inline(always)]
    pub fn BITMAP_WORDS(&self) -> StgWord {
        unimplemented!()
    }
}


#[repr(C)]
pub struct StgLargeBitmap {
    pub size    : StgWord,
    pub bitmap  : &'static StgWord
}


// info tables

#[repr(C)]
pub union StgClosureInfo {
    pub payload : {
        ptrs : StgHalfWord,  /* number of pointers */
        nptrs : StgHalfWord  /* number of non-pointers */
    },

    pub bitmap : Bitmap,
    pub selector_offset : StgWord
}

type StgClosureInfoType = StgClosureInfo;

impl StgClosureInfo {
    // TODO: pattern matching read?
}



#[repr(C)]
pub struct StgInfoTable {
    pub layout  : StgClosureInfo,
    pub type_   : StgClosureType,
    pub srt     : StgHalfWord, // what to do with SRT?
    pub code : &'static StgWord8 // TODO: StgCode
}


// function info tables

#[repr(C)]
pub struct StgFunInfoExtraRev {
    // TODO: OFFSET_FIELD(slow_apply_offset),
    pub bitmap : Bitmap,
    pub fun_type : StgFunType,
    pub arity : StgHalfWord
}

// TODO: StgFunInfoExtraFwd

#[repr(C)]
pub struct StgFunInfoTable {
    pub f : StgFunInfoExtraRev,
    pub i : StgInfoTable
}


// return info tables
pub struct StgRetInfoTable(StgInfoTable);

// thunk info tables
pub struct StgThunkInfoTable(StgInfoTable);

// Constructor info tables
pub struct StgConInfoTable {
    // offset field
    pub i : StgInfoTable
}
