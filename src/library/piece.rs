use std::fmt::format;

use crate::library::board::BoardState;

use super::player::PlayerColor;

#[derive(PartialEq, Clone, Copy)]
pub enum PieceType {
    KING,
    QUEEN,
    BISHOP,
    KNIGHT,
    ROOK,
    PAWN,
}

#[derive(PartialEq, Clone, Copy)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: PlayerColor,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: PlayerColor) -> Self {
        Self {
            piece_type: piece_type,
            color: color,
        }
    }
    pub fn img_path(&self) -> String {
        let ch = match self.color {
            PlayerColor::WHITE => 'w',
            PlayerColor::BLACK => 'b',
        };
        match &self.piece_type {
            PieceType::KING => format!("img/{}_king.png", ch),
            PieceType::QUEEN => format!("img/{}_queen.png", ch),
            PieceType::BISHOP => format!("img/{}_bishop.png", ch),
            PieceType::KNIGHT => format!("img/{}_knight.png", ch),
            PieceType::ROOK => format!("img/{}_rook.png", ch),
            PieceType::PAWN => format!("img/{}_pawn.png", ch),
        }
    }
    pub fn are_friendly(p1: &Piece, p2: &Piece) -> bool {
        p1.color == p2.color
    }
}
