use core::panic;

use crate::board::BitBoard;
use crate::board::Pieces;
use crate::board::Position;
use crate::board::Sides;
use crate::board::State;

pub struct Move {
    pub piece: usize,
    pub from: u8,
    pub to: u8,
    pub promotion: Option<Pieces>,
    pub capture: Option<Pieces>,
}

pub struct MoveGenerator {}
/* 
impl MoveGenerator {
    pub fn generate_available_moves(position: &Position, mv: Move) -> BitBoard {
        return match mv.piece {
            Pieces::PAWN => {
                if (position.state.half_move_counter == 0 || half_move_counter == 1)) {
                    BitBoard(0)
                } else {
                    BitBoard(0)
                }
                BitBoard(0)
            }
            _ => {
                panic!("Not implemented yet");
            }
        };
    }

    pub fn capture() {
        panic!("Not implemented yet");
    }

    pub fn attack() {
        panic!("Not implemented yet");
    }
}
*/