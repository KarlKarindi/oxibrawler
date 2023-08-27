use crate::board::{BitBoard, Castling, CastlingRights, Position, Sides, Square, State};
use fen;
use fen::BoardState;

pub struct FenParser {}

impl FenParser {
    pub fn parse_fen(&self, fen: &str) -> Position {
        let fen_board_state = fen::BoardState::from_fen(fen).unwrap();
        let en_passant_square = match fen_board_state.en_passant_square {
            Some(pos) => Square::from_usize(FenParser::convert_index(pos as usize)),
            None => None,
        };
        let side_to_move = match fen_board_state.side_to_play {
            fen::Color::White => Sides::WHITE,
            fen::Color::Black => Sides::BLACK,
        };

        let state = State {
            castling_rights: FenParser::find_castling_rights(&fen_board_state),
            en_passant_square,
            side_to_move,
            half_move_counter: fen_board_state.halfmove_clock,
        };

        // The fen crate parser is kind of broken and parses pieces in the wrong order.
        // This is why the colors, kings & queens are swapped when constructing the bitboards.
        let bb_pieces = [
            [
                FenParser::parse_to_bitboard(
                    &fen_board_state,
                    fen::PieceKind::Pawn,
                    fen::Color::Black,
                ),
                FenParser::parse_to_bitboard(
                    &fen_board_state,
                    fen::PieceKind::Knight,
                    fen::Color::Black,
                ),
                FenParser::parse_to_bitboard(
                    &fen_board_state,
                    fen::PieceKind::Bishop,
                    fen::Color::Black,
                ),
                FenParser::parse_to_bitboard(
                    &fen_board_state,
                    fen::PieceKind::Rook,
                    fen::Color::Black,
                ),
                FenParser::parse_to_bitboard(
                    &fen_board_state,
                    fen::PieceKind::King,
                    fen::Color::Black,
                ),
                FenParser::parse_to_bitboard(
                    &fen_board_state,
                    fen::PieceKind::Queen,
                    fen::Color::Black,
                ),
            ],
            [
                FenParser::parse_to_bitboard(
                    &fen_board_state,
                    fen::PieceKind::Pawn,
                    fen::Color::White,
                ),
                FenParser::parse_to_bitboard(
                    &fen_board_state,
                    fen::PieceKind::Knight,
                    fen::Color::White,
                ),
                FenParser::parse_to_bitboard(
                    &fen_board_state,
                    fen::PieceKind::Bishop,
                    fen::Color::White,
                ),
                FenParser::parse_to_bitboard(
                    &fen_board_state,
                    fen::PieceKind::Rook,
                    fen::Color::White,
                ),
                FenParser::parse_to_bitboard(
                    &fen_board_state,
                    fen::PieceKind::King,
                    fen::Color::White,
                ),
                FenParser::parse_to_bitboard(
                    &fen_board_state,
                    fen::PieceKind::Queen,
                    fen::Color::White,
                ),
            ],
        ];

        Position::new(state, bb_pieces)
    }

    pub fn new() -> FenParser {
        FenParser {}
    }

    // Converts indices from the fen crate to indices used in the bitboard crate.
    fn convert_index(idx: usize) -> usize {
        (7 - idx / 8) * 8 + (idx % 8)
    }

    fn find_castling_rights(board: &BoardState) -> CastlingRights {
        let mut castling_rights = CastlingRights::none();
        if board.white_can_oo {
            castling_rights.0 |= Castling::WHITE_OO;
        }
        if board.white_can_ooo {
            castling_rights.0 |= Castling::WHITE_OOO
        }
        if board.black_can_oo {
            castling_rights.0 |= Castling::BLACK_OO;
        }
        if board.black_can_ooo {
            castling_rights.0 |= Castling::BLACK_OOO;
        }
        castling_rights
    }

    fn parse_to_bitboard(
        fen_board_state: &BoardState,
        piece_kind: fen::PieceKind,
        piece_color: fen::Color,
    ) -> BitBoard {
        let mut bitboard = BitBoard(0);

        for (idx, piece_option) in fen_board_state.pieces.iter().enumerate() {
            if let Some(piece) = piece_option {
                if piece.kind == piece_kind && piece.color == piece_color {
                    bitboard.0 |= 1 << FenParser::convert_index(idx);
                }
            }
        }

        bitboard
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_fen_any_castling() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let fen_parser = FenParser::new();
        let position = fen_parser.parse_fen(fen);
        assert_eq!(
            position.state.castling_rights,
            CastlingRights(Castling::ANY_CASTLING)
        );
    }

    #[test]
    fn test_parse_fen_only_white_can_castle() {
        let fen_parser = FenParser::new();
        // White can castle king-side, but not queen-side. Black can't castle.
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w K - 0 1";
        let position = fen_parser.parse_fen(fen);
        assert_eq!(
            position.state.castling_rights,
            CastlingRights(Castling::WHITE_OO & !Castling::WHITE_OOO & !Castling::BLACK_CASTLING)
        );

        // White can castle queen-side, but not king-side. Black can't castle.
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w Q - 0 1";
        let position = fen_parser.parse_fen(fen);
        assert_eq!(
            position.state.castling_rights,
            CastlingRights(Castling::WHITE_OOO & !Castling::WHITE_OO & !Castling::BLACK_CASTLING)
        );

        // White can castle both sides. Black can't castle.
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQ - 0 1";
        let position = fen_parser.parse_fen(fen);
        assert_eq!(
            position.state.castling_rights,
            CastlingRights(Castling::WHITE_CASTLING & !Castling::BLACK_CASTLING)
        );
    }

    #[test]
    fn test_parse_fen_only_black_can_castle() {
        let fen_parser = FenParser::new();
        // Black can castle king-side, but not queen-side. White can't castle.
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b k - 0 1";
        let position = fen_parser.parse_fen(fen);
        assert_eq!(
            position.state.castling_rights,
            CastlingRights(Castling::BLACK_OO & !Castling::BLACK_OOO & !Castling::WHITE_CASTLING)
        );

        // Black can castle queen-side, but not king-side. White can't castle.
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b q - 0 1";
        let position = fen_parser.parse_fen(fen);
        assert_eq!(
            position.state.castling_rights,
            CastlingRights(Castling::BLACK_OOO & !Castling::BLACK_OO & !Castling::WHITE_CASTLING)
        );

        // Black can castle both sides. White can't castle.
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b kq - 0 1";
        let position = fen_parser.parse_fen(fen);
        assert_eq!(
            position.state.castling_rights,
            CastlingRights(Castling::BLACK_CASTLING & !Castling::WHITE_CASTLING)
        );
    }

    #[test]
    fn test_parse_fen_no_castling() {
        let fen_parser = FenParser::new();
        // Neither side can castle.
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1";
        let position = fen_parser.parse_fen(fen);
        assert_eq!(
            position.state.castling_rights,
            CastlingRights(Castling::NO_CASTLING)
        );
    }

    #[test]
    fn test_parse_fen_en_passant() {
        let fen_parser = FenParser::new();
        let fen = "rnbqkbnr/pppp1ppp/8/4P3/8/8/PPPP1PPP/RNBQKBNR b KQkq a1 0 1";
        let position = fen_parser.parse_fen(fen);
        assert_eq!(position.state.en_passant_square, Some(Square::A1));

        let fen = "rnbqkbnr/pppp1ppp/8/4P3/8/8/PPPP1PPP/RNBQKBNR b KQkq h8 0 1";
        let position = fen_parser.parse_fen(fen);
        assert_eq!(position.state.en_passant_square, Some(Square::H8));
    }

    #[test]
    fn test_convert_index() {
        assert_eq!(Square::A1 as usize, FenParser::convert_index(0));
        assert_eq!(Square::A8 as usize, 0);
        assert_eq!(Square::B1 as usize, FenParser::convert_index(1));
        assert_eq!(Square::B8 as usize, 1);
        assert_eq!(Square::H8 as usize, FenParser::convert_index(63));
        assert_eq!(Square::H1 as usize, 63);
        assert_eq!(Square::E1 as usize, FenParser::convert_index(4));
        assert_eq!(Square::E1 as usize, 60);
    }

    #[test]
    fn test_parse_position_pieces_from_fen() {
        let fen_parser = FenParser::new();
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let position = fen_parser.parse_fen(fen);
        assert_eq!(Position::default().bb_pieces, position.bb_pieces);
    }

    #[test]
    fn test_parse_position_state_from_fen() {
        let fen_parser = FenParser::new();
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let position = fen_parser.parse_fen(fen);
        assert_eq!(Position::default().state, position.state);
    }

    // TODO: Test with random game state
}
