use crate::library::piece::{MoveError, Piece};
use crate::library::player_state::{PlayerColor, PlayerState};
use crate::library::point::Point;
use yewdux::prelude::*;

static MAX_BOARD_WIDTH: u8 = 8;

#[derive(Store, PartialEq)]
pub struct BoardState {
    turn: PlayerColor,
    pub w_player: PlayerState,
    pub b_player: PlayerState,
}

impl BoardState {
    pub fn play(&self, point: &Point<u8>) -> Result<(), MoveError> {
        Ok(())
    }
    pub fn in_bounds(point: &Point<u8>) -> bool {
        if point.x > MAX_BOARD_WIDTH || point.y > MAX_BOARD_WIDTH {
            return false;
        } else {
            return true;
        }
    }
}

impl Default for BoardState {
    fn default() -> Self {
        BoardState {
            turn: PlayerColor::WHITE,
            w_player: PlayerState::new(PlayerColor::WHITE),
            b_player: PlayerState::new(PlayerColor::BLACK),
        }
    }
}
