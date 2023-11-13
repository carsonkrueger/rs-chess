use std::fmt::format;

use crate::library::{
    board::BoardState,
    point::{Dist, Point},
};

use super::player::PlayerColor;

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
    pub fn valid_move(p1: &Point<u8, Piece>, p2: &Point<u8, Piece>) -> bool {
        if p1.data.is_none() {
            return false;
        }
        if p2.data.is_some() && Self::are_friendly(&p1.data.unwrap(), &p2.data.unwrap()) {
            return false;
        }
        true
        // match p1.data.unwrap().piece_type {
        //     PieceType::KING => {
        //         true
        //     }
        //     _ => true,
        // }
    }
    pub fn img_path(&self) -> String {
        let ch = match self.color {
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
    pub fn are_friendly(p1: &Piece, p2: &Piece) -> bool {
        p1.color == p2.color
    }
}
