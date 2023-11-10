use super::piece::PieceType;
use crate::library::{piece::Piece, point::Point};
use collect_slice::CollectSlice;

#[derive(PartialEq, Clone, Copy)]
pub enum PlayerColor {
    WHITE,
    BLACK,
}

#[derive(PartialEq)]
pub struct PlayerState {
    pub color: PlayerColor,
    pub pieces: [Option<Piece>; 16],
}

pub enum PlayerStateError {
    NoPiece,
    InvalidIndex,
}

impl PlayerState {
    pub fn new(color: PlayerColor) -> PlayerState {
        PlayerState {
            color: color,
            pieces: [
                Some(Piece::new(PieceType::KING, color)),
                Some(Piece::new(PieceType::QUEEN, color)),
                Some(Piece::new(PieceType::ROOK1, color)),
                Some(Piece::new(PieceType::ROOK2, color)),
                Some(Piece::new(PieceType::BISHOP1, color)),
                Some(Piece::new(PieceType::BISHOP2, color)),
                Some(Piece::new(PieceType::KNIGHT1, color)),
                Some(Piece::new(PieceType::KNIGHT2, color)),
                Some(Piece::new(PieceType::PAWN1, color)),
                Some(Piece::new(PieceType::PAWN2, color)),
                Some(Piece::new(PieceType::PAWN3, color)),
                Some(Piece::new(PieceType::PAWN4, color)),
                Some(Piece::new(PieceType::PAWN5, color)),
                Some(Piece::new(PieceType::PAWN6, color)),
                Some(Piece::new(PieceType::PAWN7, color)),
                Some(Piece::new(PieceType::PAWN8, color)),
            ],
        }
    }
    fn piece_at_point(&self, point: &Point<u8>) -> &Option<Piece> {
        self.pieces
            .iter()
            .find(|&p| p.is_some() && &p.as_ref().unwrap().point == point)
            .unwrap()
    }
    fn piece_index(piece_type: &PieceType) -> usize {
        match piece_type {
            PieceType::KING => 0,
            PieceType::QUEEN => 1,
            PieceType::ROOK1 => 2,
            PieceType::ROOK2 => 3,
            PieceType::BISHOP1 => 4,
            PieceType::BISHOP2 => 5,
            PieceType::KNIGHT1 => 6,
            PieceType::KNIGHT2 => 7,
            PieceType::PAWN1 => 8,
            PieceType::PAWN2 => 9,
            PieceType::PAWN3 => 10,
            PieceType::PAWN4 => 11,
            PieceType::PAWN5 => 12,
            PieceType::PAWN6 => 13,
            PieceType::PAWN7 => 14,
            PieceType::PAWN8 => 15,
        }
    }
    fn piece_type(index: usize) -> Result<PieceType, PlayerStateError> {
        match index {
            0 => Ok(PieceType::KING),
            1 => Ok(PieceType::QUEEN),
            2 => Ok(PieceType::ROOK1),
            3 => Ok(PieceType::ROOK2),
            4 => Ok(PieceType::BISHOP1),
            5 => Ok(PieceType::BISHOP2),
            6 => Ok(PieceType::KNIGHT1),
            7 => Ok(PieceType::KNIGHT2),
            8 => Ok(PieceType::PAWN1),
            9 => Ok(PieceType::PAWN2),
            10 => Ok(PieceType::PAWN3),
            11 => Ok(PieceType::PAWN4),
            12 => Ok(PieceType::PAWN5),
            13 => Ok(PieceType::PAWN6),
            14 => Ok(PieceType::PAWN7),
            15 => Ok(PieceType::PAWN8),
            _ => Err(PlayerStateError::InvalidIndex),
        }
    }
    pub fn remove_piece(&mut self, point: &Point<u8>) -> Result<(), PlayerStateError> {
        match self.piece_at_point(point) {
            Some(p) => {
                self.pieces[PlayerState::piece_index(&p.piece_type)] = None;
                Ok(())
            }
            None => Err(PlayerStateError::NoPiece),
        }
    }
    fn piece(&self, piece_type: &PieceType) -> &Option<Piece> {
        &self.pieces[Self::piece_index(piece_type)]
    }
}
