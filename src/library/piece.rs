use std::fmt::format;

use crate::library::{
    board::BoardState,
    point::{Dist, Point},
};

use super::player_state::PlayerColor;

#[derive(PartialEq, Clone, Copy)]
pub enum PieceType {
    KING,
    QUEEN,
    BISHOP1,
    BISHOP2,
    KNIGHT1,
    KNIGHT2,
    ROOK1,
    ROOK2,
    PAWN1,
    PAWN2,
    PAWN3,
    PAWN4,
    PAWN5,
    PAWN6,
    PAWN7,
    PAWN8,
}

#[derive(PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub point: Point<u8>,
}

pub enum MoveError {
    OUTOFBOUNDS,
    INVALIDMOVE,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: PlayerColor) -> Piece {
        let y = match color {
            PlayerColor::WHITE => 0,
            PlayerColor::BLACK => 7,
        };
        let y2 = match color {
            PlayerColor::WHITE => 1,
            PlayerColor::BLACK => 6,
        };

        Piece {
            piece_type: piece_type,
            point: match piece_type {
                PieceType::ROOK1 => Point::from((0, y)),
                PieceType::KNIGHT1 => Point::from((1, y)),
                PieceType::BISHOP1 => Point::from((2, y)),
                PieceType::QUEEN => Point::from((3, y)),
                PieceType::KING => Point::from((4, y)),
                PieceType::BISHOP2 => Point::from((5, y)),
                PieceType::KNIGHT2 => Point::from((6, y)),
                PieceType::ROOK2 => Point::from((7, y)),
                PieceType::PAWN1 => Point::from((0, y2)),
                PieceType::PAWN2 => Point::from((1, y2)),
                PieceType::PAWN3 => Point::from((2, y2)),
                PieceType::PAWN4 => Point::from((3, y2)),
                PieceType::PAWN5 => Point::from((4, y2)),
                PieceType::PAWN6 => Point::from((5, y2)),
                PieceType::PAWN7 => Point::from((6, y2)),
                PieceType::PAWN8 => Point::from((7, y2)),
            },
        }
    }
    pub fn set_point(&mut self, point: Point<u8>) {
        self.point = point;
    }
    pub fn img_path(&self, color: &PlayerColor) -> String {
        let ch = match &color {
            PlayerColor::WHITE => 'w',
            PlayerColor::BLACK => 'b',
        };
        match &self.piece_type {
            PieceType::KING => format!("img/{}_king.png", ch),
            PieceType::QUEEN => format!("img/{}_queen.png", ch),
            PieceType::BISHOP1 | PieceType::BISHOP2 => format!("img/{}_bishop.png", ch),
            PieceType::KNIGHT1 | PieceType::KNIGHT2 => format!("img/{}_knight.png", ch),
            PieceType::ROOK1 | PieceType::ROOK2 => format!("img/{}_rook.png", ch),
            PieceType::PAWN1
            | PieceType::PAWN2
            | PieceType::PAWN3
            | PieceType::PAWN4
            | PieceType::PAWN5
            | PieceType::PAWN6
            | PieceType::PAWN7
            | PieceType::PAWN8 => format!("img/{}_pawn.png", ch),
        }
    }
}
