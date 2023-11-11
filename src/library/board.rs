use crate::library::piece::{MoveError, Piece};
use crate::library::player_state::{PlayerColor, PlayerState};
use crate::library::point::Point;
use yewdux::prelude::*;

use super::piece::PieceType;

static MAX_BOARD_WIDTH: u8 = 8;

#[derive(Store, PartialEq)]
pub struct BoardState {
    pub turn: PlayerColor,
    pub points: [Point<u8, Piece>; 64],
}

impl BoardState {
    pub fn play(&self, point: &Point<u8, Piece>) -> Result<(), MoveError> {
        Ok(())
    }
    pub fn in_bounds(point: &Point<u8, Piece>) -> bool {
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
            points: [
                Point::new(Piece::new(PieceType::ROOK1, PlayerColor::WHITE)),
                Point::new(Piece::new(PieceType::KNIGHT1, PlayerColor::WHITE)),
                Point::new(Piece::new(PieceType::BISHOP1, PlayerColor::WHITE)),
                Point::new(Piece::new(PieceType::KING, PlayerColor::WHITE)),
                Point::new(Piece::new(PieceType::QUEEN, PlayerColor::WHITE)),
                Point::new(Piece::new(PieceType::BISHOP2, PlayerColor::WHITE)),
                Point::new(Piece::new(PieceType::KNIGHT2, PlayerColor::WHITE)),
                Point::new(Piece::new(PieceType::ROOK2, PlayerColor::WHITE)),
                Point::new(Piece::new(PieceType::PAWN1, PlayerColor::WHITE)),
                Point::new(Piece::new(PieceType::PAWN2, PlayerColor::WHITE)),
                Point::new(Piece::new(PieceType::PAWN3, PlayerColor::WHITE)),
                Point::new(Piece::new(PieceType::PAWN4, PlayerColor::WHITE)),
                Point::new(Piece::new(PieceType::PAWN5, PlayerColor::WHITE)),
                Point::new(Piece::new(PieceType::PAWN6, PlayerColor::WHITE)),
                Point::new(Piece::new(PieceType::PAWN7, PlayerColor::WHITE)),
                Point::new(Piece::new(PieceType::PAWN8, PlayerColor::WHITE)),
                Point::from((0, 2)),
                Point::from((1, 2)),
                Point::from((2, 2)),
                Point::from((3, 2)),
                Point::from((4, 2)),
                Point::from((5, 2)),
                Point::from((6, 2)),
                Point::from((7, 2)),
                Point::from((0, 3)),
                Point::from((1, 3)),
                Point::from((2, 3)),
                Point::from((3, 3)),
                Point::from((4, 3)),
                Point::from((5, 3)),
                Point::from((6, 3)),
                Point::from((7, 3)),
                Point::from((0, 4)),
                Point::from((1, 4)),
                Point::from((2, 4)),
                Point::from((3, 4)),
                Point::from((4, 4)),
                Point::from((5, 4)),
                Point::from((6, 4)),
                Point::from((7, 4)),
                Point::from((0, 5)),
                Point::from((1, 5)),
                Point::from((2, 5)),
                Point::from((3, 5)),
                Point::from((4, 5)),
                Point::from((5, 5)),
                Point::from((6, 5)),
                Point::from((7, 5)),
                Point::new(Piece::new(PieceType::PAWN1, PlayerColor::BLACK)),
                Point::new(Piece::new(PieceType::PAWN2, PlayerColor::BLACK)),
                Point::new(Piece::new(PieceType::PAWN3, PlayerColor::BLACK)),
                Point::new(Piece::new(PieceType::PAWN4, PlayerColor::BLACK)),
                Point::new(Piece::new(PieceType::PAWN5, PlayerColor::BLACK)),
                Point::new(Piece::new(PieceType::PAWN6, PlayerColor::BLACK)),
                Point::new(Piece::new(PieceType::PAWN7, PlayerColor::BLACK)),
                Point::new(Piece::new(PieceType::PAWN8, PlayerColor::BLACK)),
                Point::new(Piece::new(PieceType::ROOK1, PlayerColor::BLACK)),
                Point::new(Piece::new(PieceType::KNIGHT1, PlayerColor::BLACK)),
                Point::new(Piece::new(PieceType::BISHOP1, PlayerColor::BLACK)),
                Point::new(Piece::new(PieceType::KING, PlayerColor::BLACK)),
                Point::new(Piece::new(PieceType::QUEEN, PlayerColor::BLACK)),
                Point::new(Piece::new(PieceType::BISHOP2, PlayerColor::BLACK)),
                Point::new(Piece::new(PieceType::KNIGHT2, PlayerColor::BLACK)),
                Point::new(Piece::new(PieceType::ROOK2, PlayerColor::BLACK)),
            ],
        }
    }
}
