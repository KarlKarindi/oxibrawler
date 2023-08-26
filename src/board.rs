#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Default, Hash)]
pub struct BitBoard(pub u64);

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct Position {
    // State of the game
    state: State,
    // Bitboard for either side
    // First side corresponds to WHITE, second to BLACK
    bb_sides: [BitBoard; 2],
    // Bitboards for both pieces on either side
    // First bitboards correspond to WHITE, second to BLACK
    bb_pieces: [[BitBoard; 6]; 2],
}

impl Default for Position {
    fn default() -> Self {
        let white_pieces: u64 =
            0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_11111111;
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

        let black_pieces: u64 =
            0b11111111_11111111_00000000_00000000_00000000_00000000_00000000_00000000;
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
            bb_sides: [BitBoard(white_pieces), BitBoard(black_pieces)],
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

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
/// Represents a single square on the board.
/// # Representation
/// 1 is A1
/// 2 is B1
/// 64 is H8
pub struct Square(usize);

#[repr(usize)]
#[rustfmt::skip]
pub enum SquareLabels {
    A8, B8, C8, D8, E8, F8, G8, H8,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A1, B1, C1, D1, E1, F1, G1, H1, 
    None
}

pub struct Side;

impl Side {
    pub const WHITE: usize = 0;
    pub const BLACK: usize = 1;
}

pub struct Piece;

impl Piece {
    pub const PAWN: usize = 0;
    pub const KNIGHT: usize = 1;
    pub const BISHOP: usize = 2;
    pub const ROOK: usize = 3;
    pub const QUEEN: usize = 4;
    pub const KING: usize = 5;
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct State {
    castling_rights: CastlingRights,
    en_passant_square: Option<Square>,
    half_move_counter: u8,
    side_to_move: usize,
}

impl Default for State {
    fn default() -> Self {
        Self {
            castling_rights: CastlingRights::default(),
            en_passant_square: None,
            half_move_counter: 0,
            side_to_move: Side::WHITE,
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct CastlingRights(u8);

impl CastlingRights {
    fn none() -> Self {
        Self(Castling::NO_CASTLING)
    }
}

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
