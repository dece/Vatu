//! Move functions along with some castling helpers.

use std::fmt;

use crate::board::*;
use crate::castling::*;
use crate::rules::GameState;

/// A movement, with before/after positions and optional promotion.
#[derive(Clone, PartialEq)]
pub struct Move {
    /// Square from which a piece moves.
    pub source: Square,
    /// Square to which a piece moves.
    pub dest: Square,
    /// Promotion piece for pawns reaching the last rank.
    pub promotion: Option<Piece>,
    /// Captured piece, if any.
    pub capture: Option<Piece>,
    /// Castle options before the move. This is set when the move is first applied.
    pub old_castles: Castle,
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_uci_string())
    }
}

/// Null move string in UCI exchanges.
pub const UCI_NULL_MOVE_STR: &str = "0000";

impl Move {
    /// Build a move from `source` to `dest`, no promotion.
    pub const fn new(source: Square, dest: Square) -> Move {
        Move { source, dest, promotion: None, capture: None, old_castles: 0 }
    }

    /// Build a move from `source` to `dest`, with a promotion.
    pub const fn new_promotion(source: Square, dest: Square, promotion: Piece) -> Move {
        Move { source, dest, promotion: Some(promotion), capture: None, old_castles: 0 }
    }

    /// Apply this move to `board` and `game_state`.
    ///
    /// Set automatic queen promotion for pawns, register captured
    /// pieces and castle options.
    pub fn apply_to(&mut self, board: &mut Board, game_state: &mut GameState) {
        // Save current castling options to unmake later.
        self.old_castles = game_state.castling;

        let piece = board.get_piece_on(self.source);
        // Handle king castling.
        if piece == KING {
            if let Some(castle) = self.get_castle() {
                match castle {
                    CASTLE_WH_K => {
                        board.move_square(E1, G1);
                        board.move_square(H1, F1);
                        game_state.castling &= !CASTLE_WH_MASK;
                    }
                    CASTLE_WH_Q => {
                        board.move_square(E1, C1);
                        board.move_square(A1, D1);
                        game_state.castling &= !CASTLE_WH_MASK;
                    }
                    CASTLE_BL_K => {
                        board.move_square(E8, G8);
                        board.move_square(H8, F8);
                        game_state.castling &= !CASTLE_BL_MASK;
                    }
                    CASTLE_BL_Q => {
                        board.move_square(E8, C8);
                        board.move_square(A8, D8);
                        game_state.castling &= !CASTLE_BL_MASK;
                    }
                    _ => { panic!("Invalid castle.") }
                }
                game_state.color = opposite(game_state.color);
                return
            } else {
                // If the king moved from starting square, remove it from castling options.
                if self.source == E1 { game_state.castling &= !CASTLE_WH_MASK; }
                else if self.source == E8 { game_state.castling &= !CASTLE_BL_MASK; }
            }
        }
        // Record captured piece if any.
        if !board.is_empty(self.dest) {
            self.capture = Some(board.get_piece_on(self.dest));
        }

        // Move the piece.
        board.move_square(self.source, self.dest);

        // Apply promotion if any.
        if let Some(piece) = self.promotion {
            board.set_piece(self.dest, PAWN, piece);
        }

        // If a rook moved, remove the castle side.
        if self.source == A1 || self.dest == A1 { game_state.castling &= !CASTLE_WH_Q; }
        else if self.source == H1 || self.dest == H1 { game_state.castling &= !CASTLE_WH_K; }
        else if self.source == A8 || self.dest == A8 { game_state.castling &= !CASTLE_BL_Q; }
        else if self.source == H8 || self.dest == H8 { game_state.castling &= !CASTLE_BL_K; }

        // Finally, switch to the opposing player in the game state.
        game_state.color = opposite(game_state.color);
    }

    /// Unmake a move.
    pub fn unmake(&self, board: &mut Board, game_state: &mut GameState) {
        // Always restore previous castle options.
        game_state.castling = self.old_castles;
        // If the move is a castle, unmake it properly.
        let piece = board.get_piece_on(self.dest);
        if piece == KING {
            if let Some(castle) = self.get_castle() {
                match castle {
                    CASTLE_WH_K => { board.move_square(G1, E1); board.move_square(F1, H1); }
                    CASTLE_WH_Q => { board.move_square(C1, E1); board.move_square(D1, A1); }
                    CASTLE_BL_K => { board.move_square(G8, E8); board.move_square(F8, H8); }
                    CASTLE_BL_Q => { board.move_square(C8, E8); board.move_square(D8, A8); }
                    _ => { panic!("Invalid castle.") }
                }
                game_state.color = opposite(game_state.color);
                return
            }
        }

        // Move the piece back.
        board.move_square(self.dest, self.source);

        // Cancel the promotion.
        if let Some(piece) = self.promotion {
            board.set_piece(self.source, piece, PAWN);
        }

        // Restore captured piece.
        if let Some(piece) = self.capture {
            board.set_square(self.dest, game_state.color, piece);
        }

        // And switch back to previous player.
        game_state.color = opposite(game_state.color);
    }

    /// Get the corresponding castling flag for this move.
    pub fn get_castle(&self) -> Option<Castle> {
        match (self.source, self.dest) {
            (E1, C1) => Some(CASTLE_WH_Q),
            (E1, G1) => Some(CASTLE_WH_K),
            (E8, C8) => Some(CASTLE_BL_Q),
            (E8, G8) => Some(CASTLE_BL_K),
            _ => None,
        }
    }

    /// Get the move for this castle.
    pub fn get_castle_move(castle: u8) -> Move {
        match castle {
            CASTLE_WH_Q => Move::new(E1, C1),
            CASTLE_WH_K => Move::new(E1, G1),
            CASTLE_BL_Q => Move::new(E8, C8),
            CASTLE_BL_K => Move::new(E8, G8),
            _ => panic!("Illegal castling requested: {:08b}", castle),
        }
    }

    /// Parse an UCI move algebraic notation string to a Move.
    pub fn from_uci_string(m_str: &str) -> Move {
        Move {
            source: sq_from_string(&m_str[0..2]),
            dest: sq_from_string(&m_str[2..4]),
            promotion: if m_str.len() == 5 {
                Some(match m_str.as_bytes()[4] {
                    b'b' => BISHOP,
                    b'n' => KNIGHT,
                    b'r' => ROOK,
                    b'q' => QUEEN,
                    _ => panic!("What is the opponent doing? This is illegal, I'm out."),
                })
            } else {
                None
            },
            capture: None,
            old_castles: 0,
        }
    }

    /// Create a string containing the UCI algebraic notation of this move.
    pub fn to_uci_string(&self) -> String {
        let mut move_string = String::new();
        move_string.push_str(&sq_to_string(self.source));
        move_string.push_str(&sq_to_string(self.dest));
        if let Some(piece) = self.promotion {
            move_string.push(match piece {
                QUEEN => 'q',
                BISHOP => 'b',
                KNIGHT => 'n',
                ROOK => 'r',
                _ => panic!("What are you doing? Promote to a legal piece.")
            });
        }
        move_string
    }

    /// Debug only: create a space-separated string of moves.
    pub(crate) fn list_to_uci_string(moves: &Vec<Move>) -> String {
        moves.iter().map(|m| m.to_uci_string()).collect::<Vec<_>>().join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_to_board() {
        let mut b = Board::new_empty();
        let mut gs = GameState::new();

        // Put 2 enemy knights on board.
        b.set_square(D4, WHITE, KNIGHT);
        b.set_square(F4, BLACK, KNIGHT);
        // Move white knight in a position attacked by black knight.
        let mut m = Move::new(D4, E6);
        m.apply_to(&mut b, &mut gs);
        assert!(b.is_empty(D4));
        assert_eq!(b.get_color_on(E6), WHITE);
        assert_eq!(b.get_piece_on(E6), KNIGHT);
        assert_eq!(count_bits(b.combined()), 2);
        assert!(m.capture.is_none());
        // Sack it with black knight
        let mut m = Move::new(F4, E6);
        m.apply_to(&mut b, &mut gs);
        assert_eq!(b.get_color_on(E6), BLACK);
        assert_eq!(b.get_piece_on(E6), KNIGHT);
        assert_eq!(count_bits(b.combined()), 1);
        assert_eq!(m.capture.unwrap(), KNIGHT);
    }

    #[test]
    fn test_apply_to_castling() {
        let mut b = Board::new();
        let mut gs = GameState::new();
        assert_eq!(gs.castling, CASTLE_MASK);

        // On a starting board, start by making place for all castles.
        b.clear_square(B1, WHITE, KNIGHT);
        b.clear_square(C1, WHITE, BISHOP);
        b.clear_square(D1, WHITE, QUEEN);
        b.clear_square(F1, WHITE, BISHOP);
        b.clear_square(G1, WHITE, KNIGHT);
        b.clear_square(B8, BLACK, KNIGHT);
        b.clear_square(C8, BLACK, BISHOP);
        b.clear_square(D8, BLACK, QUEEN);
        b.clear_square(F8, BLACK, BISHOP);
        b.clear_square(G8, BLACK, KNIGHT);
        // White queen-side castling.
        Move::new(E1, C1).apply_to(&mut b, &mut gs);
        assert_eq!(b.get_color_on(C1), WHITE);
        assert_eq!(b.get_piece_on(C1), KING);
        assert_eq!(b.get_color_on(D1), WHITE);
        assert_eq!(b.get_piece_on(D1), ROOK);
        assert!(b.is_empty(A1));
        assert!(b.is_empty(E1));
        assert_eq!(gs.castling, CASTLE_BL_MASK);
        // Black king-side castling.
        Move::new(E8, G8).apply_to(&mut b, &mut gs);
        assert_eq!(b.get_color_on(G8), BLACK);
        assert_eq!(b.get_piece_on(G8), KING);
        assert_eq!(b.get_color_on(F8), BLACK);
        assert_eq!(b.get_piece_on(F8), ROOK);
        assert!(b.is_empty(H8));
        assert!(b.is_empty(E8));
        // At the end, no more castling options for both sides.
        assert_eq!(gs.castling, 0);
    }

    #[test]
    fn test_unmake() {
        let mut b = Board::new_empty();
        let mut gs = GameState::new();

        // On unmaking a move, the white pawn is back to its original square.
        b.set_square(D4, WHITE, PAWN);
        let mut m = Move::new(D4, D5);
        m.apply_to(&mut b, &mut gs);
        m.unmake(&mut b, &mut gs);
        assert!(b.is_empty(D5));
        assert_eq!(b.get_color_on(D4), WHITE);
        assert_eq!(b.get_piece_on(D4), PAWN);

        // Castle options should be properly unmade.
        b.set_square(E1, WHITE, KING);
        b.set_square(H1, WHITE, ROOK);
        let mut m = Move::new(E1, G1);
        m.apply_to(&mut b, &mut gs);
        assert!(!b.is_empty(G1));
        assert!(!b.is_empty(F1));
        assert_eq!(gs.castling, CASTLE_MASK ^ CASTLE_WH_MASK);
        m.unmake(&mut b, &mut gs);
        assert!(b.is_empty(G1));
        assert!(b.is_empty(F1));
        assert_eq!(b.get_piece_on(E1), KING);
        assert_eq!(b.get_piece_on(H1), ROOK);
        assert_eq!(gs.castling, CASTLE_MASK);
    }

    #[test]
    fn test_get_castle() {
        assert_eq!(Move::new(E1, C1).get_castle(), Some(CASTLE_WH_Q));
        assert_eq!(Move::new(E1, G1).get_castle(), Some(CASTLE_WH_K));
        assert_eq!(Move::new(E8, C8).get_castle(), Some(CASTLE_BL_Q));
        assert_eq!(Move::new(E8, G8).get_castle(), Some(CASTLE_BL_K));
        assert_eq!(Move::new(D2, D4).get_castle(), None);
    }

    #[test]
    fn test_to_uci_string() {
        assert_eq!(Move::new(A1, D4).to_uci_string(), "a1d4");
        assert_eq!(Move::new(H8, A8).to_uci_string(), "h8a8");
        assert_eq!(Move::new_promotion(H7, H8, QUEEN).to_uci_string(), "h7h8q");
        assert_eq!(Move::new_promotion(H7, H8, KNIGHT).to_uci_string(), "h7h8n");
    }

    #[test]
    fn test_from_uci_string() {
        assert_eq!(Move::from_uci_string("a1d4"), Move::new(A1, D4));
        assert_eq!(Move::from_uci_string("a7a8q"), Move::new_promotion(A7, A8, QUEEN));
        assert_eq!(Move::from_uci_string("a7a8r"), Move::new_promotion(A7, A8, ROOK));
    }
}
