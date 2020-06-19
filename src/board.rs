//! Basic type definitions and functions.

/// Bitboard for color or piece bits.
pub type Bitboard = u64;

/// Color type, used to index `Board.color`.
pub type Color = usize;

pub const WHITE: usize = 0;
pub const BLACK: usize = 1;
pub const NUM_COLORS: usize = 2;

/// Get opposite color.
#[inline]
pub const fn opposite(color: Color) -> Color { color ^ 1 }

/// Pretty-print a color.
pub fn color_to_string(color: Color) -> String {
    match color {
        0 => "white".to_string(),
        1 => "black".to_string(),
        _ => panic!("Unknown color {}", color),
    }
}

/// Piece type, used to index `Board.piece`.
pub type Piece = usize;

pub const PAWN: usize = 0;
pub const BISHOP: usize = 1;
pub const KNIGHT: usize = 2;
pub const ROOK: usize = 3;
pub const QUEEN: usize = 4;
pub const KING: usize = 5;
pub const NUM_PIECES: usize = 6;

/// Coords (file, rank) of a square on a board.
pub type Square = i8;

// Generated by gen_squares.py.
pub const A1: Square = 0;
pub const A2: Square = 1;
pub const A3: Square = 2;
pub const A4: Square = 3;
pub const A5: Square = 4;
pub const A6: Square = 5;
pub const A7: Square = 6;
pub const A8: Square = 7;
pub const B1: Square = 8;
pub const B2: Square = 9;
pub const B3: Square = 10;
pub const B4: Square = 11;
pub const B5: Square = 12;
pub const B6: Square = 13;
pub const B7: Square = 14;
pub const B8: Square = 15;
pub const C1: Square = 16;
pub const C2: Square = 17;
pub const C3: Square = 18;
pub const C4: Square = 19;
pub const C5: Square = 20;
pub const C6: Square = 21;
pub const C7: Square = 22;
pub const C8: Square = 23;
pub const D1: Square = 24;
pub const D2: Square = 25;
pub const D3: Square = 26;
pub const D4: Square = 27;
pub const D5: Square = 28;
pub const D6: Square = 29;
pub const D7: Square = 30;
pub const D8: Square = 31;
pub const E1: Square = 32;
pub const E2: Square = 33;
pub const E3: Square = 34;
pub const E4: Square = 35;
pub const E5: Square = 36;
pub const E6: Square = 37;
pub const E7: Square = 38;
pub const E8: Square = 39;
pub const F1: Square = 40;
pub const F2: Square = 41;
pub const F3: Square = 42;
pub const F4: Square = 43;
pub const F5: Square = 44;
pub const F6: Square = 45;
pub const F7: Square = 46;
pub const F8: Square = 47;
pub const G1: Square = 48;
pub const G2: Square = 49;
pub const G3: Square = 50;
pub const G4: Square = 51;
pub const G5: Square = 52;
pub const G6: Square = 53;
pub const G7: Square = 54;
pub const G8: Square = 55;
pub const H1: Square = 56;
pub const H2: Square = 57;
pub const H3: Square = 58;
pub const H4: Square = 59;
pub const H5: Square = 60;
pub const H6: Square = 61;
pub const H7: Square = 62;
pub const H8: Square = 63;

/// Get bit mask of `p` in a bitboard.
#[inline]
pub const fn bit_pos(square: Square) -> u64 { 1 << square }

/// Convert string coordinates to Square.
///
/// `s` has to be valid UTF8, or the very least ASCII because chars
/// are interpreted as raw bytes, and lowercase.
#[inline]
pub const fn sq_from_string(square: &str) -> Square {
    let chars = square.as_bytes();
    (chars[0] - 0x61) as i8 * 8 + (chars[1] - 0x31) as i8
}

/// Return string coordinates from Square.
pub fn sq_to_string(square: Square) -> String {
    let mut bytes = [0u8; 2];
    bytes[0] = ((square / 8) + 0x61) as u8;
    bytes[1] = ((square % 8) + 0x31) as u8;
    String::from_utf8_lossy(&bytes).to_string()
}

/// Board representation with color/piece bitboards.
#[derive(Clone, PartialEq)]
pub struct Board {
    pub colors: [Bitboard; 2],
    pub pieces: [Bitboard; 6],
}

// Factories.
impl Board {
    /// Generate the board of a new game.
    pub const fn new() -> Board {
        Board {
            colors: [
                0b11000000_11000000_11000000_11000000_11000000_11000000_11000000_11000000,
                0b00000011_00000011_00000011_00000011_00000011_00000011_00000011_00000011,
            ],
            pieces: [
                0b01000010_01000010_01000010_01000010_01000010_01000010_01000010_01000010,
                0b00000000_00000000_10000001_00000000_00000000_10000001_00000000_00000000,
                0b00000000_10000001_00000000_00000000_00000000_00000000_10000001_00000000,
                0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_10000001,
                0b00000000_00000000_00000000_10000001_00000000_00000000_00000000_00000000,
                0b00000000_00000000_00000000_00000000_10000001_00000000_00000000_00000000,
            ]
        }
    }

    /// Generate an empty board.
    pub const fn new_empty() -> Board {
        Board {
            colors: [0; 2],
            pieces: [0; 6],
        }
    }

    /// Generate a board from a FEN placement string.
    pub fn new_from_fen(fen: &str) -> Board {
        let mut board = Board::new_empty();
        let mut f = 0;
        let mut r = 7;
        for c in fen.chars() {
            match c {
                'r' => { board.set_square(f * 8 + r, BLACK, ROOK); f += 1 }
                'n' => { board.set_square(f * 8 + r, BLACK, KNIGHT); f += 1 }
                'b' => { board.set_square(f * 8 + r, BLACK, BISHOP); f += 1 }
                'q' => { board.set_square(f * 8 + r, BLACK, QUEEN); f += 1 }
                'k' => { board.set_square(f * 8 + r, BLACK, KING); f += 1 }
                'p' => { board.set_square(f * 8 + r, BLACK, PAWN); f += 1 }
                'R' => { board.set_square(f * 8 + r, WHITE, ROOK); f += 1 }
                'N' => { board.set_square(f * 8 + r, WHITE, KNIGHT); f += 1 }
                'B' => { board.set_square(f * 8 + r, WHITE, BISHOP); f += 1 }
                'Q' => { board.set_square(f * 8 + r, WHITE, QUEEN); f += 1 }
                'K' => { board.set_square(f * 8 + r, WHITE, KING); f += 1 }
                'P' => { board.set_square(f * 8 + r, WHITE, PAWN); f += 1 }
                '/' => { f = 0; r -= 1; }
                d if d.is_digit(10) => { f += d.to_digit(10).unwrap() as i8 }
                _ => break,
            }
        }
        board
    }
}

impl Board {
    /// Get combined white/black pieces bitboard.
    #[inline]
    pub fn combined(&self) -> Bitboard {
        self.colors[WHITE] | self.colors[BLACK]
    }

    /// True if this square is empty.
    pub fn is_empty(&self, square: Square) -> bool {
        self.combined() & bit_pos(square) == 0
    }

    /// Get color type at position. It must hold a piece!
    pub fn get_color(&self, square: Square) -> Color {
        let bp = bit_pos(square);
        if (self.colors[WHITE] & bp) == 1 { WHITE }
        else if (self.pieces[BLACK] & bp) == 1 { BLACK }
        else { panic!("Empty square.") }
    }

    /// Get piece type at position. It must hold a piece!
    pub fn get_piece(&self, square: Square) -> Piece {
        let bp = bit_pos(square);
        if (self.pieces[PAWN] & bp) == 1 { PAWN }
        else if (self.pieces[BISHOP] & bp) == 1 { BISHOP }
        else if (self.pieces[KNIGHT] & bp) == 1 { KNIGHT }
        else if (self.pieces[ROOK] & bp) == 1 { ROOK }
        else if (self.pieces[QUEEN] & bp) == 1 { QUEEN }
        else if (self.pieces[KING] & bp) == 1 { KING }
        else { panic!("Empty square.") }
    }

    /// Set a new value for the square at this position.
    #[inline]
    pub fn set_square(&mut self, square: Square, color: Color, piece: Piece) {
        self.colors[color] |= bit_pos(square);
        self.pieces[piece] |= bit_pos(square);
    }

    /// Set the square empty at this position.
    #[inline]
    pub fn clear_square(&mut self, square: Square) {
        for color in 0..NUM_COLORS { self.colors[color] &= !bit_pos(square); }
        for piece in 0..NUM_PIECES { self.pieces[piece] &= !bit_pos(square); }
    }

    /// Move a piece from a position to another, clearing initial position.
    #[inline]
    pub fn move_square(&mut self, source: Square, dest: Square) {
        self.set_square(dest, self.get_color(source), self.get_piece(source));
        self.clear_square(source);
    }

    /// Find position of this king.
    pub fn find_king(&self, color: Color) -> Option<Square> {
        let king_bb = self.colors[color] & self.pieces[KING];
        for square in 0..64 {
            if king_bb & bit_pos(square) == 1 {
                return Some(square)
            }
        }
        None
    }

    /// Debug only: count number of pieces on board.
    pub fn num_pieces(&self) -> u8 {
        let cbb = self.combined();
        let mut count = 0;
        while cbb > 0 {
            count += cbb & 1;
            cbb >>= 1;
        }
        0
    }

    /// Debug only: write a text view of the board.
    pub fn draw(&self, f: &mut dyn std::io::Write) {
        let cbb = self.colors[WHITE] | self.colors[BLACK];
        for rank in (0..8).rev() {
            let mut rank_str = String::with_capacity(8);
            for file in 0..8 {
                let square = file * 8 + rank;
                let bp = bit_pos(square);
                let piece_char = if cbb & bp == 0 {
                    '.'
                } else {
                    let (color, piece) = (self.get_color(square), self.get_piece(square));
                    let mut piece_char = match piece {
                        PAWN => 'p',
                        BISHOP => 'b',
                        KNIGHT => 'n',
                        ROOK => 'r',
                        QUEEN => 'q',
                        KING => 'k',
                    };
                    if color == WHITE {
                        let piece_char = piece_char.to_ascii_uppercase();
                    }
                    piece_char
                };
                rank_str.push(piece_char);
            }
            writeln!(f, "{} {}", rank + 1, rank_str).unwrap();
        }
        write!(f, "  abcdefgh").unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::notation;

    #[test]
    fn test_opposite() {
        assert_eq!(opposite(WHITE), BLACK);
        assert_eq!(opposite(BLACK), WHITE);
    }

    #[test]
    fn test_sq_from_string() {
        assert_eq!(sq_from_string("a1"), A1);
        assert_eq!(sq_from_string("a2"), A2);
        assert_eq!(sq_from_string("a8"), A8);
        assert_eq!(sq_from_string("b1"), B1);
        assert_eq!(sq_from_string("h8"), H8);
    }

    #[test]
    fn test_sq_to_string() {
        assert_eq!(sq_to_string(A1), "a1");
        assert_eq!(sq_to_string(A2), "a2");
        assert_eq!(sq_to_string(A8), "a8");
        assert_eq!(sq_to_string(H8), "h8");
    }

    #[test]
    fn test_new_from_fen() {
        let b1 = Board::new();
        let b2 = Board::new_from_fen(notation::FEN_START);
        assert!(b1 == b2);
    }

    #[test]
    fn test_get_color() {
        let b = Board::new();
        assert_eq!(b.get_color(A1), WHITE);
        assert_eq!(b.get_color(A2), WHITE);
        assert_eq!(b.get_color(A7), BLACK);
        assert_eq!(b.get_color(A8), BLACK);
        assert_eq!(b.get_color(D1), WHITE);
        assert_eq!(b.get_color(D8), BLACK);
        assert_eq!(b.get_color(E1), WHITE);
        assert_eq!(b.get_color(E8), BLACK);
    }

    #[test]
    fn test_get_piece() {
        let b = Board::new();
        assert_eq!(b.get_piece(A1), ROOK);
        assert_eq!(b.get_piece(A2), PAWN);
        assert_eq!(b.get_piece(A7), PAWN);
        assert_eq!(b.get_piece(A8), ROOK);
        assert_eq!(b.get_piece(D1), QUEEN);
        assert_eq!(b.get_piece(D8), QUEEN);
        assert_eq!(b.get_piece(E1), KING);
        assert_eq!(b.get_piece(E8), KING);
    }

    #[test]
    fn test_find_king() {
        let b = Board::new_empty();
        assert_eq!(b.find_king(WHITE), None);
        let b = Board::new();
        assert_eq!(b.find_king(WHITE), Some(E1));
        assert_eq!(b.find_king(BLACK), Some(E8));
    }

    #[test]
    fn test_num_pieces() {
        assert_eq!(Board::new_empty().num_pieces(), 0);
        assert_eq!(Board::new().num_pieces(), 32);
    }
}
