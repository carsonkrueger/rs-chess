use super::piece::PieceType;
use crate::library::piece::Piece;
use collect_slice::CollectSlice;

#[derive(PartialEq, Clone, Copy)]
pub enum PlayerColor {
    WHITE,
    BLACK,
}

#[derive(PartialEq)]
pub struct PlayerState {
    pub color: PlayerColor,
}
