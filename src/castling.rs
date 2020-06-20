//! Castling flags.

pub type Castle = u8;

pub const CASTLING_WH_K: Castle    = 0b00000001;
pub const CASTLING_WH_Q: Castle    = 0b00000010;
pub const CASTLING_WH_MASK: Castle = 0b00000011;
pub const CASTLING_BL_K: Castle    = 0b00000100;
pub const CASTLING_BL_Q: Castle    = 0b00001000;
pub const CASTLING_BL_MASK: Castle = 0b00001100;
pub const CASTLING_K_MASK: Castle  = 0b00000101;
pub const CASTLING_Q_MASK: Castle  = 0b00001010;
pub const CASTLING_MASK: Castle    = 0b00001111;

/// Castling sides parameters.
///
/// For both sides, the 3-uple contains files that should be empty
/// and not attacked, an optional file that should be empty for
/// queen-side, and the castling side-mask.
pub const CASTLING_SIDES: [([i8; 2], Option<i8>, Castle); 2] =
    [([5i8, 6i8], None, CASTLING_K_MASK), ([3i8, 2i8], Some(1i8), CASTLING_Q_MASK)];
