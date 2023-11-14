use crate::library::piece::Piece;
use crate::library::player::{PlayerColor, PlayerState};
use crate::library::point::Point;
use yewdux::prelude::*;

use super::piece::PieceType;

static MAX_BOARD_WIDTH: usize = 8;

#[derive(Store, PartialEq, Clone, Copy)]
pub struct BoardState {
    pub turn: PlayerColor,
    pub points: [Option<Piece>; 64],
    pub select1_idx: Option<usize>,
    pub select2_idx: Option<usize>,
}

#[derive(Debug)]
pub enum MoveError {
    OutOfBounds,
    InvalidMove,
    NoPiece,
    InvalidIndex,
}

impl BoardState {
    pub fn play(&self) -> Result<(), MoveError> {
        if self.select1_idx.is_none() || self.select2_idx.is_none() {
            return Err(MoveError::InvalidIndex);
        } else if !Self::in_bounds(self.select1_idx.unwrap())
            || !Self::in_bounds(self.select2_idx.unwrap())
        {
            return Err(MoveError::OutOfBounds);
        }

        let mut p1 = self.points[self.select1_idx.unwrap()];
        let mut p2 = self.points[self.select2_idx.unwrap()];

        if p1.is_none() {
            return Err(MoveError::NoPiece);
        } else if !Piece::valid_move(
            p1.unwrap(),
            self.select1_idx.unwrap(),
            self.select2_idx.unwrap(),
        ) {
            return Err(MoveError::InvalidMove);
        }

        p2 = p1.clone(); // move piece to second point
        p1 = None; // clear initial piece

        Ok(())
    }
    pub fn in_bounds(idx: usize) -> bool {
        if idx > MAX_BOARD_WIDTH || idx > MAX_BOARD_WIDTH {
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
            select1_idx: None,
            select2_idx: None,
            points: [
                Some(Piece::new(PieceType::ROOK1, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::KNIGHT1, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::BISHOP1, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::KING, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::QUEEN, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::BISHOP2, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::KNIGHT2, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::ROOK2, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::PAWN1, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::PAWN2, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::PAWN3, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::PAWN4, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::PAWN5, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::PAWN6, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::PAWN7, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::PAWN8, PlayerColor::WHITE)),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Piece::new(PieceType::PAWN1, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::PAWN2, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::PAWN3, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::PAWN4, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::PAWN5, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::PAWN6, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::PAWN7, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::PAWN8, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::ROOK1, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::KNIGHT1, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::BISHOP1, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::KING, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::QUEEN, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::BISHOP2, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::KNIGHT2, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::ROOK2, PlayerColor::BLACK)),
            ],
        }
    }
}
