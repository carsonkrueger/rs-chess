use num_traits::Num;

use crate::library::piece::{Piece, PieceType};
use crate::library::player::PlayerColor;

#[derive(PartialEq, Copy, Clone)]
pub struct Point<T: Num + Copy, U> {
    pub x: T,
    pub y: T,
    pub data: Option<U>,
}

pub struct Dist<T: Num + Copy> {
    pub x: T,
    pub y: T,
}

impl<T: Num + Copy, U> From<(T, T)> for Point<T, U> {
    fn from(value: (T, T)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            data: None,
        }
    }
}

impl Point<u8, Piece> {
    pub fn new(piece: Piece) -> Self {
        let y = match piece.color {
            PlayerColor::WHITE => 0,
            PlayerColor::BLACK => 7,
        };
        let y2 = match piece.color {
            PlayerColor::WHITE => 1,
            PlayerColor::BLACK => 6,
        };

        let xy = match piece.piece_type {
            PieceType::ROOK1 => (0, y),
            PieceType::KNIGHT1 => (1, y),
            PieceType::BISHOP1 => (2, y),
            PieceType::QUEEN => (3, y),
            PieceType::KING => (4, y),
            PieceType::BISHOP2 => (5, y),
            PieceType::KNIGHT2 => (6, y),
            PieceType::ROOK2 => (7, y),
            PieceType::PAWN1 => (0, y2),
            PieceType::PAWN2 => (1, y2),
            PieceType::PAWN3 => (2, y2),
            PieceType::PAWN4 => (3, y2),
            PieceType::PAWN5 => (4, y2),
            PieceType::PAWN6 => (5, y2),
            PieceType::PAWN7 => (6, y2),
            PieceType::PAWN8 => (7, y2),
        };

        Self {
            x: xy.0,
            y: xy.1,
            data: Some(piece),
        }
    }
}

impl<U> Point<u8, U> {
    // pub fn set_point(&mut self, point: Point<u8, U>) {
    //     self.x = point.x;
    //     self.y = point.y;
    // }
    pub fn relative_point_dist(&self, point: &Point<u8, U>) -> Dist<i32> {
        Dist {
            x: self.x as i32 - point.x as i32,
            y: self.y as i32 - point.y as i32,
        }
    }
}
