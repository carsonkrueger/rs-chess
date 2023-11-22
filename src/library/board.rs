use std::rc::Rc;

use crate::library::piece::Piece;
use crate::library::player::{PlayerColor, PlayerState};
use crate::library::point::Point;
use crate::Board;
use yewdux::prelude::*;

use super::piece::PieceType;

static MAX_BOARD_WIDTH: usize = 8;

#[derive(Store, PartialEq, Clone, Copy)]
pub struct BoardState {
    pub turn: PlayerColor,
    pub points: [Option<Piece>; 64],
    pub select1_idx: Option<usize>,
    pub select2_idx: Option<usize>,
}

#[derive(Debug)]
pub enum MoveError {
    OutOfBounds,
    InvalidMove,
    NoPiece,
    InvalidIndex,
}

pub enum Action {
    UpdateBoardState(Rc<BoardState>),
}

impl BoardState {
    pub fn reduce(&mut self, action: Action) {
        match action {
            Action::UpdateBoardState(new_state) => *self = (*new_state).clone(),
        }
    }
    pub fn play(&mut self) -> Result<(), MoveError> {
        if self.select1_idx.is_none() || self.select2_idx.is_none() {
            return Err(MoveError::InvalidIndex);
        } else if !Self::in_bounds(self.select1_idx.unwrap())
            || !Self::in_bounds(self.select2_idx.unwrap())
        {
            return Err(MoveError::OutOfBounds);
        }

        let mut p1 = self.points[self.select1_idx.unwrap()];
        let mut p2 = self.points[self.select2_idx.unwrap()];

        if p1.is_none() {
            return Err(MoveError::NoPiece);
        } else if !self.valid_move(self.select1_idx.unwrap(), self.select2_idx.unwrap()) {
            return Err(MoveError::InvalidMove);
        } else if p2.is_some() && Piece::are_friendly(&p1.unwrap(), &p2.unwrap()) {
            return Err(MoveError::InvalidMove);
        }

        // p2 = p1.clone(); // move piece to second point
        // p1 = None; // clear initial piece

        self.points[self.select1_idx.unwrap()] = None;
        self.points[self.select2_idx.unwrap()] = p1.clone();

        Ok(())
    }
    pub fn in_bounds(idx: usize) -> bool {
        // if idx > MAX_BOARD_WIDTH || idx > MAX_BOARD_WIDTH {
        if idx >= MAX_BOARD_WIDTH * MAX_BOARD_WIDTH {
            return false;
        } else {
            return true;
        }
    }
    pub fn valid_move(&self, from: usize, to: usize) -> bool {
        let p_from = self.points[from].as_ref().unwrap();
        let p_to = self.points[to].as_ref();
        match p_from.piece_type {
            PieceType::PAWN1
            | PieceType::PAWN2
            | PieceType::PAWN3
            | PieceType::PAWN4
            | PieceType::PAWN5
            | PieceType::PAWN6
            | PieceType::PAWN7
            | PieceType::PAWN8 => {
                (Self::is_forward(from, to, p_from.color) && p_to.is_none())
                    || (p_to.is_some()
                        && Self::is_adjacent_diagnol_forward(from, to, p_from.color)
                        && !Piece::are_friendly(p_from, p_to.unwrap()))
            }
            PieceType::KNIGHT1 | PieceType::KNIGHT2 => Self::is_knight_hop(from, to),
            PieceType::BISHOP1 | PieceType::BISHOP2 => Self::is_diagnol(from, to),
            PieceType::ROOK1 | PieceType::ROOK2 => Self::is_slide(from, to),
            PieceType::KING => Self::is_adjacent(from, to),
            PieceType::QUEEN => Self::is_diagnol(from, to) || Self::is_slide(from, to),
            _ => true,
        }
    }
    fn is_adjacent(from: usize, to: usize) -> bool {
        match from as i32 - to as i32 {
            8 => true,
            -8 => true,
            1 => true,
            -1 => true,
            -9 => true,
            -7 => true,
            9 => true,
            7 => true,
            _ => false,
        }
    }
    fn is_adjacent_diagnol_forward(from: usize, to: usize, color: PlayerColor) -> bool {
        match color {
            PlayerColor::WHITE => match from as i32 - to as i32 {
                9 => true,
                7 => true,
                _ => false,
            },
            PlayerColor::BLACK => match from as i32 - to as i32 {
                -7 => true,
                -9 => true,
                _ => false,
            },
        }
    }
    fn is_diagnol(from: usize, to: usize) -> bool {
        let dist = from as i32 - to as i32;
        dist % 9 == 0 || dist % 7 == 0
    }
    fn is_knight_hop(from: usize, to: usize) -> bool {
        match from as i32 - to as i32 {
            17 => true,
            15 => true,
            -17 => true,
            -15 => true,
            -10 => true,
            -6 => true,
            10 => true,
            6 => true,
            _ => false,
        }
    }
    fn is_slide(from: usize, to: usize) -> bool {
        let dist = from as i32 - to as i32;
        dist % 8 == 0 || from / 8 == to / 8
    }
    fn is_forward(from: usize, to: usize, color: PlayerColor) -> bool {
        match color {
            PlayerColor::WHITE => from + 8 == to,
            PlayerColor::BLACK => from - 8 == to,
        }
    }
}

impl Default for BoardState {
    fn default() -> Self {
        BoardState {
            turn: PlayerColor::WHITE,
            select1_idx: None,
            select2_idx: None,
            points: [
                Some(Piece::new(PieceType::ROOK1, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::KNIGHT1, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::BISHOP1, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::KING, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::QUEEN, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::BISHOP2, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::KNIGHT2, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::ROOK2, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::PAWN1, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::PAWN2, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::PAWN3, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::PAWN4, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::PAWN5, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::PAWN6, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::PAWN7, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::PAWN8, PlayerColor::WHITE)),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Piece::new(PieceType::PAWN1, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::PAWN2, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::PAWN3, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::PAWN4, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::PAWN5, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::PAWN6, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::PAWN7, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::PAWN8, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::ROOK1, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::KNIGHT1, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::BISHOP1, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::KING, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::QUEEN, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::BISHOP2, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::KNIGHT2, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::ROOK2, PlayerColor::BLACK)),
            ],
        }
    }
}
