use core::panic;

use crate::board::BitBoard;
use crate::board::Pieces;
use crate::board::Position;
use crate::board::Sides;
use crate::board::Square;

pub struct Move {
    pub piece: usize,
    pub from: Square,
    pub to: Square,
    pub promotion: Option<Pieces>,
    pub capture: Option<Pieces>,
}

impl Move {
    pub fn new(piece: usize, from: Square, to: Square) -> Move {
        Move {
            piece,
            from,
            to,
            promotion: None,
            capture: None,
        }
    }
}

pub struct MoveGenerator {}
impl MoveGenerator {

    pub fn generate_available_moves(position: &Position) -> BitBoard {
        return match position.state.side_to_move {
            Sides::WHITE => {
                MoveGenerator::generate_available_moves_for_side(position, Sides::WHITE)
            }
            Sides::BLACK => {
                MoveGenerator::generate_available_moves_for_side(position, Sides::BLACK)
            }
            _ => panic!("Invalid side to move"),
        }
    }

    pub fn capture() {
        panic!("Not implemented yet");
    }

    fn generate_available_moves_for_side(position: &Position, side: usize) -> BitBoard {
        let mut available_moves = BitBoard::empty();
        let mut pawns = position.bb_pieces[side][Pieces::PAWN];
        // Iter over pawns
        /* for pawn in pawns.iter() {
            let pawn_square = pawn.square();
            let pawn_moves = MoveGenerator::generate_pawn_moves(position, pawn_square, side);
            available_moves = available_moves | pawn_moves;
        } */
        
        return available_moves;
    }
}