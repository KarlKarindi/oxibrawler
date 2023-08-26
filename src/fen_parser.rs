use crate::board::{BitBoard, Castling, CastlingRights, Position, Sides, State};
use fen;
use fen::BoardState;

pub struct FenParser {}

impl FenParser {
    pub fn parse_fen(&self, fen: &str) -> Position {
        let board = fen::BoardState::from_fen(fen).unwrap();
        let state = State {
            castling_rights: FenParser::find_castling_rights(&board),
            en_passant_square: None,    // FIXME
            side_to_move: Sides::WHITE, // FIXME
            half_move_counter: board.halfmove_clock as u64,
        };

        // FIXME: Parse pieces
        let bb_pieces = [
            [
                BitBoard(0),
                BitBoard(0),
                BitBoard(0),
                BitBoard(0),
                BitBoard(0),
                BitBoard(0),
            ],
            [
                BitBoard(0),
                BitBoard(0),
                BitBoard(0),
                BitBoard(0),
                BitBoard(0),
                BitBoard(0),
            ],
        ];

        Position::new(state, bb_pieces)
    }

    pub fn new() -> FenParser {
        FenParser {}
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
}
