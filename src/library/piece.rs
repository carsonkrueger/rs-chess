use std::fmt::format;

use crate::library::board::BoardState;

use super::player::PlayerColor;

#[derive(PartialEq, Clone, Copy)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(PartialEq, Clone, Copy)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: PlayerColor,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: PlayerColor) -> Self {
        Self { piece_type, color }
    }
    pub fn img_path(&self) -> String {
        let ch = match self.color {
            PlayerColor::White => 'w',
            PlayerColor::Black => 'b',
        };
        match &self.piece_type {
            PieceType::King => format!("img/{}_king.png", ch),
            PieceType::Queen => format!("img/{}_queen.png", ch),
            PieceType::Bishop => format!("img/{}_bishop.png", ch),
            PieceType::Knight => format!("img/{}_knight.png", ch),
            PieceType::Rook => format!("img/{}_rook.png", ch),
            PieceType::Pawn => format!("img/{}_pawn.png", ch),
        }
    }
    pub fn are_friendly(p1: &Piece, p2: &Piece) -> bool {
        p1.color == p2.color
    }
}
