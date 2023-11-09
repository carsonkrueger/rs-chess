use crate::components::piece::Piece;

pub enum PlayerColor {
    WHITE,
    BLACK,
}

pub struct PlayerState {
    pub color: PlayerColor,
    pub king: Option<Piece>,
    pub queen: Option<Piece>,
    pub bishop1: Option<Piece>,
    pub bishop2: Option<Piece>,
    pub rook1: Option<Piece>,
    pub rook2: Option<Piece>,
    pub knight1: Option<Piece>,
    pub knight2: Option<Piece>,
    pub pawn1: Option<Piece>,
    pub pawn2: Option<Piece>,
    pub pawn3: Option<Piece>,
    pub pawn4: Option<Piece>,
    pub pawn5: Option<Piece>,
    pub pawn6: Option<Piece>,
    pub pawn7: Option<Piece>,
    pub pawn8: Option<Piece>,
}
