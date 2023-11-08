use crate::library::{board::BoardState, point::Point};
use num_traits::Num;

#[derive(PartialEq)]
pub enum ChessPieceType {
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

impl ChessPieceType {
    pub fn img_path(&self) -> String {
        match &self {
            ChessPieceType::WKING => String::from("img/w_king.png"),
            ChessPieceType::WQUEEN => String::from("img/w_queen.png"),
            ChessPieceType::WBISHOP => String::from("img/w_bishop.png"),
            ChessPieceType::WKNIGHT => String::from("img/w_knight.png"),
            ChessPieceType::WROOK => String::from("img/w_rook.png"),
            ChessPieceType::WPAWN => String::from("img/w_pawn.png"),
            ChessPieceType::BKING => String::from("img/b_king.png"),
            ChessPieceType::BQUEEN => String::from("img/b_queen.png"),
            ChessPieceType::BBISHOP => String::from("img/b_bishop.png"),
            ChessPieceType::BKNIGHT => String::from("img/b_knight.png"),
            ChessPieceType::BROOK => String::from("img/b_rook.png"),
            ChessPieceType::BPAWN => String::from("img/b_pawn.png"),
        }
    }
}

pub struct ChessPiece<'a, T: Num + Copy> {
    board: &'a BoardState,
    piece_type: ChessPieceType,
    point: Option<Point<T>>,
}

impl<T: Num + Copy> ChessPiece<'_, T> {
    fn move_to(&self, np: Point<T>) -> Result<(), ()> {
        if !self.board.valid_move(self.piece_type, np) {
            return Err(());
        }
        Ok(())
    }
    pub fn valid_move(&self, piece_type: ChessPieceType, point: &Point<u8>) -> bool {
        match piece_type {
            ChessPieceType::WKING => self.valid_king_move(point),
            _ => false,
        }
    }
    fn valid_king_move(&self, point: &Point<u8>) -> bool {
        if !Self::in_bounds(point) {
            return false;
        }
        match self.point.relative_point_dist(point) {
            Point { x: 1, y: 1 } => true,
            Point { x: 0, y: 1 } => true,
            Point { x: 1, y: 0 } => true,
            Point { x: -1, y: -1 } => true,
            Point { x: 0, y: -1 } => true,
            Point { x: -1, y: 0 } => true,
            _ => false,
        }
    }
}
