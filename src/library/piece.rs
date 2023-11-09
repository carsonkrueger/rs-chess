use crate::library::{
    board::BoardState,
    point::{Dist, Point},
};

#[derive(PartialEq)]
pub enum PieceType {
    WKING,
    WQUEEN,
    WBISHOP,
    WKNIGHT,
    WROOK,
    WPAWN,
    BKING,
    BQUEEN,
    BBISHOP,
    BKNIGHT,
    BROOK,
    BPAWN,
}

impl PieceType {
    pub fn img_path(&self) -> String {
        match &self {
            PieceType::WKING => String::from("img/w_king.png"),
            PieceType::WQUEEN => String::from("img/w_queen.png"),
            PieceType::WBISHOP => String::from("img/w_bishop.png"),
            PieceType::WKNIGHT => String::from("img/w_knight.png"),
            PieceType::WROOK => String::from("img/w_rook.png"),
            PieceType::WPAWN => String::from("img/w_pawn.png"),
            PieceType::BKING => String::from("img/b_king.png"),
            PieceType::BQUEEN => String::from("img/b_queen.png"),
            PieceType::BBISHOP => String::from("img/b_bishop.png"),
            PieceType::BKNIGHT => String::from("img/b_knight.png"),
            PieceType::BROOK => String::from("img/b_rook.png"),
            PieceType::BPAWN => String::from("img/b_pawn.png"),
        }
    }
}

pub struct Piece<'a> {
    board: &'a BoardState,
    pub piece_type: PieceType,
    pub point: Point<u8>,
}

pub enum MoveError {
    OUTOFBOUNDS,
    INVALIDMOVE,
}

impl Piece<'_> {
    pub fn move_to(&self, np: &Point<u8>) -> Result<(), MoveError> {
        if !BoardState::in_bounds(np) {
            return Err(MoveError::OUTOFBOUNDS);
        } else if !self.valid_move(&self.piece_type, np) {
            return Err(MoveError::INVALIDMOVE);
        }
        Ok(())
    }
    pub fn valid_move(&self, piece_type: &PieceType, point: &Point<u8>) -> bool {
        match piece_type {
            PieceType::WKING => self.valid_king_move(point),
            _ => false,
        }
    }
    fn valid_king_move(&self, point: &Point<u8>) -> bool {
        match self.point.relative_point_dist(point) {
            Dist { x: 1, y: 1 } => true,
            Dist { x: 0, y: 1 } => true,
            Dist { x: 1, y: 0 } => true,
            Dist { x: -1, y: -1 } => true,
            Dist { x: 0, y: -1 } => true,
            Dist { x: -1, y: 0 } => true,
            _ => false,
        }
    }
}
