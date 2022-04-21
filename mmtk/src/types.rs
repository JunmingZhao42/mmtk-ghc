type StgWord = usize,
type StgHalfWord = u32, // TODO: change this size later
type StgWord16 = u16,
type StgWord8 = u8,
type StgInt = i64

// ------------ ClosureTypes.h ------------
#[derive(Eq)] // comparison traits
pub struct StgClosureType (StgHalfWord),

impl StgClosureType {
    pub const INVALID_OBJECT                 : StgClosureType = StgClosureType(0),
    pub const CONSTR                         : StgClosureType = StgClosureType(1),
    pub const CONSTR_1_0                     : StgClosureType = StgClosureType(2),
    pub const CONSTR_0_1                     : StgClosureType = StgClosureType(3),
    pub const CONSTR_2_0                     : StgClosureType = StgClosureType(4),
    pub const CONSTR_1_1                     : StgClosureType = StgClosureType(5),
    pub const CONSTR_0_2                     : StgClosureType = StgClosureType(6),
    pub const CONSTR_NOCAF                   : StgClosureType = StgClosureType(7),
    pub const FUN                            : StgClosureType = StgClosureType(8),
    pub const FUN_1_0                        : StgClosureType = StgClosureType(9),
    pub const FUN_0_1                        : StgClosureType = StgClosureType(10),
    pub const FUN_2_0                        : StgClosureType = StgClosureType(11),
    pub const FUN_1_1                        : StgClosureType = StgClosureType(12),
    pub const FUN_0_2                        : StgClosureType = StgClosureType(13),
    pub const FUN_STATIC                     : StgClosureType = StgClosureType(14),
    pub const THUNK                          : StgClosureType = StgClosureType(15),
    pub const THUNK_1_0                      : StgClosureType = StgClosureType(16),
    pub const THUNK_0_1                      : StgClosureType = StgClosureType(17),
    pub const THUNK_2_0                      : StgClosureType = StgClosureType(18),
    pub const THUNK_1_1                      : StgClosureType = StgClosureType(19),
    pub const THUNK_0_2                      : StgClosureType = StgClosureType(20),
    pub const THUNK_STATIC                   : StgClosureType = StgClosureType(21),
    pub const THUNK_SELECTOR                 : StgClosureType = StgClosureType(22),
    pub const BCO                            : StgClosureType = StgClosureType(23),
    pub const AP                             : StgClosureType = StgClosureType(24),
    pub const PAP                            : StgClosureType = StgClosureType(25),
    pub const AP_STACK                       : StgClosureType = StgClosureType(26),
    pub const IND                            : StgClosureType = StgClosureType(27),
    pub const IND_STATIC                     : StgClosureType = StgClosureType(28),
    pub const RET_BCO                        : StgClosureType = StgClosureType(29),
    pub const RET_SMALL                      : StgClosureType = StgClosureType(30),
    pub const RET_BIG                        : StgClosureType = StgClosureType(31),
    pub const RET_FUN                        : StgClosureType = StgClosureType(32),
    pub const UPDATE_FRAME                   : StgClosureType = StgClosureType(33),
    pub const CATCH_FRAME                    : StgClosureType = StgClosureType(34),
    pub const UNDERFLOW_FRAME                : StgClosureType = StgClosureType(35),
    pub const STOP_FRAME                     : StgClosureType = StgClosureType(36),
    pub const BLOCKING_QUEUE                 : StgClosureType = StgClosureType(37),
    pub const BLACKHOLE                      : StgClosureType = StgClosureType(38),
    pub const MVAR_CLEAN                     : StgClosureType = StgClosureType(39),
    pub const MVAR_DIRTY                     : StgClosureType = StgClosureType(40),
    pub const TVAR                           : StgClosureType = StgClosureType(41),
    pub const ARR_WORDS                      : StgClosureType = StgClosureType(42),
    pub const MUT_ARR_PTRS_CLEAN             : StgClosureType = StgClosureType(43),
    pub const MUT_ARR_PTRS_DIRTY             : StgClosureType = StgClosureType(44),
    pub const MUT_ARR_PTRS_FROZEN_DIRTY      : StgClosureType = StgClosureType(45),
    pub const MUT_ARR_PTRS_FROZEN_CLEAN      : StgClosureType = StgClosureType(46),
    pub const MUT_VAR_CLEAN                  : StgClosureType = StgClosureType(47),
    pub const MUT_VAR_DIRTY                  : StgClosureType = StgClosureType(48),
    pub const WEAK                           : StgClosureType = StgClosureType(49),
    pub const PRIM                           : StgClosureType = StgClosureType(50),
    pub const MUT_PRIM                       : StgClosureType = StgClosureType(51),
    pub const TSO                            : StgClosureType = StgClosureType(52),
    pub const STACK                          : StgClosureType = StgClosureType(53),
    pub const TREC_CHUNK                     : StgClosureType = StgClosureType(54),
    pub const ATOMICALLY_FRAME               : StgClosureType = StgClosureType(55),
    pub const CATCH_RETRY_FRAME              : StgClosureType = StgClosureType(56),
    pub const CATCH_STM_FRAME                : StgClosureType = StgClosureType(57),
    pub const WHITEHOLE                      : StgClosureType = StgClosureType(58),
    pub const SMALL_MUT_ARR_PTRS_CLEAN       : StgClosureType = StgClosureType(59),
    pub const SMALL_MUT_ARR_PTRS_DIRTY       : StgClosureType = StgClosureType(60),
    pub const SMALL_MUT_ARR_PTRS_FROZEN_DIRTY  : StgClosureType = StgClosureType(61),
    pub const SMALL_MUT_ARR_PTRS_FROZEN_CLEAN  : StgClosureType = StgClosureType(62),
    pub const COMPACT_NFDATA                 : StgClosureType = StgClosureType(63),
    pub const N_CLOSURE_TYPES                : StgClosureType = StgClosureType(64),
}

// ------------ FunTypes.h ------------
#[derive(Eq)]
pub struct StgFunType (StgHalfWord),

impl StgFunType {
    pub const ARG_GEN     : StgFunType = StgFunType( 0),
    pub const ARG_GEN_BIG : StgFunType = StgFunType( 1),
    pub const ARG_BCO     : StgFunType = StgFunType( 2),
    pub const ARG_NONE    : StgFunType = StgFunType( 3),
    pub const ARG_N       : StgFunType = StgFunType( 4),
    pub const ARG_P       : StgFunType = StgFunType( 5),
    pub const ARG_F       : StgFunType = StgFunType( 6),
    pub const ARG_D       : StgFunType = StgFunType( 7),
    pub const ARG_L       : StgFunType = StgFunType( 8),
    pub const ARG_V16     : StgFunType = StgFunType( 9),
    pub const ARG_V32      : StgFunType = StgFunType(10),
    pub const ARG_V64      : StgFunType = StgFunType(11),
    pub const ARG_NN       : StgFunType = StgFunType(12),
    pub const ARG_NP       : StgFunType = StgFunType(13),
    pub const ARG_PN       : StgFunType = StgFunType(14),
    pub const ARG_PP       : StgFunType = StgFunType(15),
    pub const ARG_NNN      : StgFunType = StgFunType(16),
    pub const ARG_NNP      : StgFunType = StgFunType(17),
    pub const ARG_NPN      : StgFunType = StgFunType(18),
    pub const ARG_NPP      : StgFunType = StgFunType(19),
    pub const ARG_PNN      : StgFunType = StgFunType(20),
    pub const ARG_PNP      : StgFunType = StgFunType(21),
    pub const ARG_PPN      : StgFunType = StgFunType(22),
    pub const ARG_PPP      : StgFunType = StgFunType(23),
    pub const ARG_PPPP     : StgFunType = StgFunType(24),
    pub const ARG_PPPPP    : StgFunType = StgFunType(25),
    pub const ARG_PPPPPP   : StgFunType = StgFunType(26),
    pub const ARG_PPPPPPP  : StgFunType = StgFunType(27),
    pub const ARG_PPPPPPPP : StgFunType = StgFunType(28),
}