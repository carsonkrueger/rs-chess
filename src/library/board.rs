use crate::library::{piece::ChessPieceType, point::Point};

static MAX_BOARD_WIDTH: u8 = 8;

enum ChessErr {
    OUTOFBOUNDS,
}

pub struct BoardState {
    w_turn: bool,
}

impl BoardState {
    pub fn move_to(&self, point: &Point<u8>) -> Result<(), ChessErr> {
        Ok(())
    }
    pub fn in_bounds(point: &Point<u8>) -> bool {
        if point.x < 0 || point.y < 0 || point.x > MAX_BOARD_WIDTH || point.y > MAX_BOARD_WIDTH {
            return false;
        } else {
            return true;
        }
    }
}
