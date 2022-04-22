// TODO: OFFSET_FIELD
// type StgSRTField = StgHalfInt,
// type StgCode = StgWord8

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
    // those should be closure flags.
    pub const _HNF : ClosureFlag = ClosureFlag(1<<0),  /* head normal form?    */
    pub const _BTM : ClosureFlag = ClosureFlag(1<<1),  /* uses info->layout.bitmap */
    pub const _NS  : ClosureFlag = ClosureFlag(1<<2),  /* non-sparkable        */
    pub const _THU : ClosureFlag = ClosureFlag(1<<3),  /* thunk?               */
    pub const _MUT : ClosureFlag = ClosureFlag(1<<4),  /* mutable?             */
    pub const _UPT : ClosureFlag = ClosureFlag(1<<5),  /* unpointed?           */
    pub const _SRT : ClosureFlag = ClosureFlag(1<<6),  /* has an SRT?          */
    pub const _IND : ClosureFlag = ClosureFlag(1<<7),  /* is an indirection?   */

    #[inline(always)]
    pub fn isMUTABLE(&self) -> bool {
        (self & self::_MUT)
    }

    // TOOD: continue implement flags related macros

    #[inline(always)]
    pub fn get_closure_flags(ty : StgClosureType) -> StgWord16 {
        unimplemented!()
    }

}


/* -----------------------------------------------------------------------------
   Bitmaps
   -------------------------------------------------------------------------- */

pub struct StgSmallBitmap (StgWord), // rename to small bitmap

// introduce Bitmap as 
pub union Bitmap {
    pub small_bitmap : StgSmallBitmap,
    pub large_bitmap_offset : StgInt,
}

impl Bitmap {
    // TODO: handle 32 bits constants
    pub const BITMAP_BITS_SHIFT : StgWord = 6,
    pub const BITMAP_SIZE_MASK : StgWord = 0x3f,
    pub const BITMAP_BITS_SHIFT : StgWord = 6,

    // ----- rust style naming -----

    #[inline(always)]
    pub fn MK_SMALL_BITMAP(size : StgWord, bit : StgWord) -> Self {
        (((bits)<<BITMAP_BITS_SHIFT) | (size))
    }

    // TODO: implement bitmap related macros

    #[inline(always)]
    pub fn size(&self) -> StgWord {
        unimplemented!()
    }

    #[inline(always)]
    pub fn bits(&self) -> StgWord {
        unimplemented!()
    }
}


// might want to iterate through bits

pub struct LargeBitMapPayload {}

impl LargeBitMapPayload {
    pub fn get(&self, i: usize) -> *mut StgWord {
        unsafe {
            let ptr: *const LargeBitMapPayload = &*self;
            let payload: *const *mut StgClosure = ptr.cast();
            *payload.offset(i as isize)
        }
    }
}

#[repr(C)]
pub struct StgLargeBitmap {
    pub size    : StgWord,
    pub bitmap  : LargeBitMapPayload // similar to closure payload in stg_closures.rs
}



#[repr(C)]
pub struct StgLargeBitmapRef {
    pub offset : StgInt
    // TODO: handle non TABLES_NEXT_TO_CODE
}

impl StgLargeBitmapRef {
    pub fn deref(&self, itbl: &StgInfoTable) -> *const StgLargeBitmap {
        unsafe {
            let offset: isize = self.layout.large_bitmap as isize;
            let end_of_itbl: *const u8 = (self as *const T).offset(1);
            (end_of_itbl as *const u8).offset(offset).cast()
        }
    }
}


/* ----------------------------------------------------------------------------
   Info Tables
   ------------------------------------------------------------------------- */


#[repr(C)]
pub union StgClosureInfo {
    pub payload : {
        ptrs : StgHalfWord,  /* number of pointers */
        nptrs : StgHalfWord,  /* number of non-pointers */
    }, // declare outside

    pub bitmap : StgSmallBitmap,
    
    // TODO: check if x64 is still related to OFFSET_FIELD
    // Check if hack in Note [x86-64-relative] is still necessary 
    pub large_bitmap : LargeBitmapRef,

    pub selector_offset : StgWord
}

/* ----------------------------------------------------------------------------
   Function info tables
   ------------------------------------------------------------------------- */

#[repr(C)]
pub struct StgSRTField {
    pub srt : StgHalfInt,
    // TODO: handle non USE_INLINE_SRT_FIELD
}

#[cfg(not(profiling))]
#[repr(C)]
pub struct StgProfInfo {} // TODO: handle profiling case

#[repr(C)]
pub struct StgInfoTable {
    // TODO: non TABLES_NEXT_TO_CODE
    #[cfg(not(tables_next_to_code))]
    pub code    : *const u8, // pointer to entry code
    pub prof    : StgProfInfo,
    pub layout  : StgClosureInfo,
    pub type_   : StgClosureType,
    pub srt     : StgSRTField, // what to do with SRT?
    // pub code    : *mut StgCode, (zero length array)
}


#[repr(C)]
pub struct StgFunInfoExtra {
    pub slow_apply : StgInt,

    pub bitmap : Bitmap,

    // TODO: handle offset for USE_INLINE_SRT_FIELD
    // for srtfield
    pub fun_type : StgFunType, // in types.rs from rts/include/rts/storage/FunTypes.h
    pub arity : StgHalfWord,
    // TODO: handle non TABLES_NEXT_TO_CODE (StgFunInfoExtraFwd)
}


#[repr(C)]
pub struct StgFunInfoTable {
    pub f : StgFunInfoExtra,
    pub i : StgInfoTable
    // TODO: handle non TABLES_NEXT_TO_CODE (need to use StgFunInfoExtraFwd)
}

/* -----------------------------------------------------------------------------
   Return info tables
   -------------------------------------------------------------------------- */
#[repr(C)]
pub struct StgRetInfoTable {
    // (check line 160 InfoTables.h)
    // TODO: USE_SRT_POINTER is true
    // TODO: USE_SRT_POINTER is false but USE_SRT_OFFSET is true
    pub i : StgInfoTable, // both false case
}

// TODO: Handle non-INLINE_SRT_FIELD case
fn get_srt_<T>(itbl: &T) -> *const StgClosure {
    unsafe {
        let offset: isize = self.i.srt as isize;
        let end_of_itbl: *const u8 = (self as *const T).offset(1);
        (end_of_itbl as *const u8).offset(offset).cast()
    }
}

impl StgRetInfoTable {
    pub fn get_srt(&self) -> *const StgClosure {
        get_srt_(self)
    }
}

/* -----------------------------------------------------------------------------
   Thunk info tables
   -------------------------------------------------------------------------- */
#[repr(C)]
pub struct StgThunkInfoTable {
    // (check line 160 InfoTables.h)
    // TODO: USE_SRT_POINTER is true
    // TODO: USE_SRT_POINTER is false but USE_SRT_OFFSET is true
    pub i : StgInfoTable, // both false case
}

impl StgThunkInfoTable {
    pub fn get_srt(&self) -> *const StgClosure {
        get_srt_(self)
    }
}

/* -----------------------------------------------------------------------------
   Constructor info tables
   -------------------------------------------------------------------------- */
#[repr(C)]
pub struct StgConInfoTable {
    // TODO: handle non TABLES_NEXT_TO_CODE
    pub con_desc_offset : StgInt,
    pub i : StgInfoTable,
}

impl StgConInfoTable {
    pub fn get_fun_srt(&self) 
}

// TODO: implement other macros
