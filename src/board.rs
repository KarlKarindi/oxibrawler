use crate::move_generator::Move;

#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Hash)]
pub struct BitBoard(pub u64);

impl BitBoard {
    // Set the bit at position 'pos' to 1
    pub fn set_bit(&mut self, square: Square) {
        self.0 |= 1 << (63 - Square::to_usize(square));
    }

    // Set the bit at position 'pos' to 0
    pub fn clear_bit(&mut self, square: Square) {
        self.0 &= !(1 << (63 - Square::to_usize(square)));
    }

    // Toggle the bit at position 'pos'
    pub fn toggle_bit(&mut self, square: Square) {
        self.0 ^= 1 << (63 - Square::to_usize(square));
    }

    // Check if the bit at position 'pos' is 1
    pub fn is_bit_set(&self, square: Square) -> bool {
        (self.0 & (1 << (63 - Square::to_usize(square)))) != 0
    }

    pub fn empty() -> BitBoard {
        BitBoard(0)
    }

    pub fn full() -> BitBoard {
        BitBoard(u64::MAX)
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct Position {
    // State of the game
    pub state: State,
    // Bitboards for both pieces on either side
    // First bitboards correspond to WHITE, second to BLACK
    pub bb_pieces: [[BitBoard; 6]; 2],
}
impl Position {
    fn load_position_from_fen(fen: &str) -> Self {
        todo!()
    }

    pub fn find_occupied_by(&self, side: usize) -> BitBoard {
        let mut pieces = BitBoard(0);
        for piece_type in 0..6 {
            pieces.0 |= self.bb_pieces[side][piece_type].0;
        }
        pieces
    }

    pub fn find_empty(&self) -> BitBoard {
        BitBoard(
            !(Self::find_occupied_by(self, Sides::WHITE).0
                | Self::find_occupied_by(self, Sides::BLACK).0),
        )
    }

    pub fn find_occupied(&self) -> BitBoard {
        BitBoard(
            Self::find_occupied_by(self, Sides::WHITE).0
                | Self::find_occupied_by(&self, Sides::BLACK).0,
        )
    }

    pub fn make_move(&mut self, mv: Move) {
        self.state.half_move_counter += 1;
        self.state.side_to_move ^= 1;
        self.bb_pieces[self.state.side_to_move][mv.piece].clear_bit(mv.from);
        self.bb_pieces[self.state.side_to_move][mv.piece].set_bit(mv.to);

        // TODO: Verify if this is correct
    }
}

impl Default for Position {
    fn default() -> Self {
        let white_pawns: u64 =
            0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000;
        let white_knights: u64 =
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_01000010;
        let white_bishops: u64 =
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100100;
        let white_rooks: u64 =
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000001;
        let white_queens: u64 =
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010000;
        let white_king: u64 =
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000;

        let black_pawns: u64 =
            0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000;
        let black_knights: u64 =
            0b01000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
        let black_bishops: u64 =
            0b00100100_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
        let black_rooks: u64 =
            0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
        let black_queens: u64 =
            0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
        let black_king: u64 =
            0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;

        Self {
            state: State::default(),
            bb_pieces: [
                [
                    BitBoard(white_pawns),
                    BitBoard(white_knights),
                    BitBoard(white_bishops),
                    BitBoard(white_rooks),
                    BitBoard(white_queens),
                    BitBoard(white_king),
                ],
                [
                    BitBoard(black_pawns),
                    BitBoard(black_knights),
                    BitBoard(black_bishops),
                    BitBoard(black_rooks),
                    BitBoard(black_queens),
                    BitBoard(black_king),
                ],
            ],
        }
    }
}

#[repr(usize)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[rustfmt::skip]
pub enum Square {
    A8 = 0, B8, C8, D8, E8, F8, G8, H8,
    A7,     B7, C7, D7, E7, F7, G7, H7,
    A6,     B6, C6, D6, E6, F6, G6, H6,
    A5,     B5, C5, D5, E5, F5, G5, H5,
    A4,     B4, C4, D4, E4, F4, G4, H4,
    A3,     B3, C3, D3, E3, F3, G3, H3,
    A2,     B2, C2, D2, E2, F2, G2, H2,
    A1,     B1, C1, D1, E1, F1, G1, H1 = 63,
}

impl Square {
    // Converts a usize between 0 and 63 to a Square.
    pub fn from_usize(val: usize) -> Option<Self> {
        if val <= 63 {
            Some(unsafe { std::mem::transmute(val) })
        } else {
            None
        }
    }

    // Converts a Square back to a usize.
    pub fn to_usize(self) -> usize {
        self as usize
    }
}

pub struct Sides;

impl Sides {
    pub const WHITE: usize = 0;
    pub const BLACK: usize = 1;
}

pub struct Pieces;

impl Pieces {
    pub const PAWN: usize = 0;
    pub const KNIGHT: usize = 1;
    pub const BISHOP: usize = 2;
    pub const ROOK: usize = 3;
    pub const QUEEN: usize = 4;
    pub const KING: usize = 5;
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct State {
    pub castling_rights: CastlingRights,
    pub en_passant_square: Option<Square>,
    pub half_move_counter: u8,
    pub side_to_move: usize,
}

impl Default for State {
    fn default() -> Self {
        Self {
            castling_rights: CastlingRights::default(),
            en_passant_square: None,
            half_move_counter: 0,
            side_to_move: Sides::WHITE,
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct CastlingRights(u8);

impl Default for CastlingRights {
    fn default() -> Self {
        Self(Castling::ANY_CASTLING)
    }
}

pub struct Castling;
impl Castling {
    pub const NO_CASTLING: u8 = 0;
    pub const WHITE_OO: u8 = 0b0001;
    pub const WHITE_OOO: u8 = 0b0010;
    pub const BLACK_OO: u8 = 0b0100;
    pub const BLACK_OOO: u8 = 0b1000;

    pub const KING_SIDE: u8 = Self::WHITE_OO | Self::BLACK_OO;
    pub const QUEEN_SIDE: u8 = Self::WHITE_OOO | Self::BLACK_OOO;
    pub const WHITE_CASTLING: u8 = Self::WHITE_OO | Self::WHITE_OOO;
    pub const BLACK_CASTLING: u8 = Self::BLACK_OO | Self::BLACK_OOO;
    pub const ANY_CASTLING: u8 = Self::KING_SIDE | Self::QUEEN_SIDE;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitboard_set_bit() {
        let mut board = BitBoard::empty();
        board.set_bit(Square::A8);
        board.set_bit(Square::A7);
        board.set_bit(Square::H1);
        assert_eq!(
            board.0,
            0b10000000_10000000_00000000_00000000_00000000_00000000_00000000_00000001
        );
    }

    #[test]
    fn test_bitboard_clear_bit() {
        let mut board = BitBoard::full();
        board.clear_bit(Square::A8);
        board.clear_bit(Square::A7);
        board.clear_bit(Square::H1);
        assert_eq!(
            board.0,
            0b01111111_01111111_11111111_11111111_11111111_11111111_11111111_11111110
        );
    }

    #[test]
    fn test_find_empty() {
        let position = Position::default();
        let empty = position.find_empty();
        assert_eq!(
            empty.0,
            0b00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000
        );
    }

    #[test]
    fn test_find_occupied() {
        let mut position = Position::default();
        position.bb_pieces[Sides::WHITE][Pieces::PAWN].clear_bit(Square::A8);
        let occupied = position.find_occupied();
        assert_eq!(
            occupied.0,
            0b11111111_11111111_00000000_00000000_00000000_00000000_11111111_11111111
        );
    }

    #[test]
    fn test_find_occupied_by_black() {
        let position = Position::default();
        let occupied = position.find_occupied_by(Sides::BLACK);
        assert_eq!(
            occupied.0,
            0b11111111_11111111_00000000_00000000_00000000_00000000_00000000_00000000
        );
    }

    #[test]
    fn test_find_occupied_by_white() {
        let position = Position::default();
        let occupied = position.find_occupied_by(Sides::WHITE);
        assert_eq!(
            occupied.0,
            0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_11111111
        );
    }

    #[test]
    fn test_position_set_bit() {
        let mut position = Position::default();
        position.bb_pieces[Sides::WHITE][Pieces::PAWN].set_bit(Square::A8);
        position.bb_pieces[Sides::WHITE][Pieces::PAWN].set_bit(Square::C7);
        assert_eq!(
            position.bb_pieces[Sides::WHITE][Pieces::PAWN].0,
            0b10000000_00100000_00000000_00000000_00000000_00000000_11111111_00000000
        );
    }

    #[test]
    fn test_position_clear_bit() {
        let mut position = Position::default();
        position.bb_pieces[Sides::BLACK][Pieces::PAWN].clear_bit(Square::A7);
        position.bb_pieces[Sides::BLACK][Pieces::PAWN].clear_bit(Square::C7);
        assert_eq!(
            position.bb_pieces[Sides::BLACK][Pieces::PAWN].0,
            0b00000000_01011111_00000000_00000000_00000000_00000000_00000000_00000000
        );
    }

    #[test]
    fn test_position_make_move() {
        let mut position = Position::default();
        /*
        position.bb_pieces[Sides::WHITE][Pieces::PAWN].make_move(

        );
        position.bb_pieces[Sides::WHITE][Pieces::PAWN].set_bit(16);
        assert_eq!(
            position.bb_pieces[Sides::WHITE][Pieces::PAWN].0,
            0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000
        ); */
    }
}
