use crate::library::piece::MoveError;
use crate::library::player_state::{PlayerColor, PlayerState};
use crate::library::point::Point;

static MAX_BOARD_WIDTH: u8 = 8;

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
        if point.x < 0 || point.y < 0 || point.x > MAX_BOARD_WIDTH || point.y > MAX_BOARD_WIDTH {
            return false;
        } else {
            return true;
        }
    }
    pub fn is_friendly_piece(&self, point: &Point<u8>, color: &PlayerColor) -> bool {
        let w = &self.w_player;
        let b = &self.b_player;
        match color {
            PlayerColor::WHITE => {
                if w.king.is_some() {
                    return true;
                }
                false
            }
            PlayerColor::BLACK => match point {
                _ => true,
            },
        }
    }
}
