use crate::library::{piece::Piece, point::Point};

use super::piece::PieceType;

pub enum PlayerColor {
    WHITE,
    BLACK,
}

pub struct PlayerState {
    pub color: PlayerColor,
    // pub king: Option<Piece>,
    // pub queen: Option<Piece>,
    // pub bishop1: Option<Piece>,
    // pub bishop2: Option<Piece>,
    // pub rook1: Option<Piece>,
    // pub rook2: Option<Piece>,
    // pub knight1: Option<Piece>,
    // pub knight2: Option<Piece>,
    // pub pawn1: Option<Piece>,
    // pub pawn2: Option<Piece>,
    // pub pawn3: Option<Piece>,
    // pub pawn4: Option<Piece>,
    // pub pawn5: Option<Piece>,
    // pub pawn6: Option<Piece>,
    // pub pawn7: Option<Piece>,
    // pub pawn8: Option<Piece>,
    pub pieces: [Option<Piece>; 16]
}

enum PlayerStateError {
    NoPiece,
}

impl PlayerState {
    // pub fn is_at_piece(&self, &)
    pub fn piece_at_point(&self, point: &Point<u8>) -> Option<&Piece> {
        let x = self.pieces.iter().find(|&p| p.is_some() && p.unwrap().point == point)
    }
    pub fn take_piece(&self, point: &Point<u8>) -> Result<(), PlayerStateError::NoPiece> {
        let p = match self.piece_at(point) {
            Some(p) => p.point,
            None => return Err(PlayerStateError::NoPiece),
        };
        Ok(())
    }
    pub fn piece(&self, piece_type: &PieceType) -> Option<Piece> {
        match piece_type => {
            PieceType::KING => self.pieces[0],
            _ => self.pieces[0]
        }
    }
}
