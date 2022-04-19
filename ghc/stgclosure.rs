type StgWord = usize;
type StgHalfWord = u32; // TODO: change this size later


// ------------ InfoTables.h ------------

#[repr(C)]
pub struct StgLargeBitmap {
    pub size    : StgWord,
    pub bitmap  : StgWord
}


#[repr(C)]
pub struct StgClosureInfo (StgWord); // TODO


#[derive(Eq)]
struct StgClosureType (StgHalfWord);

impl StgClosureType {
    pub const CONSTR: StgClosureType = StgClosureType(1);
    // TODO ... // from rts/include/rts/storage/ClosureTypes.h
}

// same for srt field

#[repr(C)]
pub struct StgInfoTable {
    pub layout  : StgClosureInfo,
    pub type_   : StgClosureType,
    pub srt     : StgHalfWord // int_32
}


// ------------ Closures.h ------------
// TODO: same for closures.h
// StgHeader
// StgClosure