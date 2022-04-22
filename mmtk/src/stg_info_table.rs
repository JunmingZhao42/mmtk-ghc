// ------------ InfoTables.h ------------

/* -----------------------------------------------------------------------------
   Profiling info
   -------------------------------------------------------------------------- */

// pub struct StgProfInfo {

// }


/* -----------------------------------------------------------------------------
   Closure flags
   -------------------------------------------------------------------------- */

#[repr(C)]
pub struct ClosureFlag (StgWord16),

impl ClosureFlag {
    // TODO: implement Flag related macro
    pub const _HNF : StgWord16 = (1<<0),  /* head normal form?    */
    pub const _BTM : StgWord16 = (1<<1),  /* uses info->layout.bitmap */
    pub const _NS  : StgWord16 = (1<<2),  /* non-sparkable        */
    pub const _THU : StgWord16 = (1<<3),  /* thunk?               */
    pub const _MUT : StgWord16 = (1<<4),  /* mutable?             */
    pub const _UPT : StgWord16 = (1<<5),  /* unpointed?           */
    pub const _SRT : StgWord16 = (1<<6),  /* has an SRT?          */
    pub const _IND : StgWord16 = (1<<7),  /* is an indirection?   */

    #[inline(always)]
    pub fn isMUTABLE(&self) -> bool {
        (self & self::_MUT)
    }

    // TOOD: continue implement flags related macros

    #[inline(always)]
    pub fn get_closure_flags(StgClosureType) -> StgWord16 {
        unimplemented!()
    }

}


/* -----------------------------------------------------------------------------
   Bitmaps
   -------------------------------------------------------------------------- */

pub struct Bitmap (StgWord),

impl Bitmap {
    pub const BITMAP_BITS_SHIFT : StgWord = 6,
    pub const BITMAP_SIZE_MASK : StgWord = 0x3f,
    pub const BITMAP_BITS_SHIFT : StgWord = 6,

    #[inline(always)]
    pub fn MK_SMALL_BITMAP(size : StgWord, bit : StgWord) -> Self {
        (((bits)<<BITMAP_BITS_SHIFT) | (size))
    }

    // TODO: implement bitmap related macros

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
    pub bitmap  : *mut StgWord,
}


/* ----------------------------------------------------------------------------
   Info Tables
   ------------------------------------------------------------------------- */

#[repr(C)]
pub union StgClosureInfo {
    pub payload : {
        ptrs : StgHalfWord,  /* number of pointers */
        nptrs : StgHalfWord  /* number of non-pointers */
    },

    pub bitmap : Bitmap,
    
    // offset

    pub selector_offset : StgWord
}

type StgClosureInfoType = StgClosureInfo;


/* ----------------------------------------------------------------------------
   Function info tables
   ------------------------------------------------------------------------- */

#[repr(C)]
pub struct StgInfoTable {
    pub layout  : StgClosureInfo,
    pub type_   : StgClosureType,
    pub srt     : StgSRTField, // what to do with SRT?
    pub code    : *mut StgWord8 
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

/* -----------------------------------------------------------------------------
   Return info tables
   -------------------------------------------------------------------------- */

// return info tables
pub struct StgRetInfoTable(StgInfoTable);

/* -----------------------------------------------------------------------------
   Thunk info tables
   -------------------------------------------------------------------------- */

// thunk info tables
pub struct StgThunkInfoTable(StgInfoTable);


/* -----------------------------------------------------------------------------
   Constructor info tables
   -------------------------------------------------------------------------- */


// Constructor info tables
pub struct StgConInfoTable {
    // offset field
    pub i : StgInfoTable
}

// TODO: implement other macros
