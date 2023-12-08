use std::rc::Rc;

use crate::library::piece::Piece;
use crate::library::player::{PlayerColor, PlayerState};
use crate::Board;
use gloo::console::log;
// use rodio::{source::Source, Decoder, OutputStream};
// use std::fs::File;
// use std::io::BufReader;
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
            Action::UpdateBoardState(new_state) => *self = (*new_state),
        }
    }
    fn play(&mut self) -> Result<(), MoveError> {
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
        } else if let (Some(i1), Some(i2)) = (p1, p2) {
            if Piece::are_friendly(&i1, &i2) {
                return Err(MoveError::InvalidMove);
            }
        }

        self.points[self.select1_idx.unwrap()] = None;
        self.points[self.select2_idx.unwrap()] = p1;

        self.try_pawn_upgrade(self.select2_idx.unwrap()); // Upgrades pawns on other side of board

        self.turn = match self.turn {
            PlayerColor::White => PlayerColor::Black,
            PlayerColor::Black => PlayerColor::White,
        };

        Ok(())
    }
    pub fn select(&mut self, idx: usize) {
        if self.select1_idx.is_none() {
            self.select1_idx = Some(idx);
        } else if self.select2_idx.is_none() {
            self.select2_idx = Some(idx);
        }

        if self.select1_idx.is_some() && self.select2_idx.is_some() {
            let i1 = self.select1_idx.unwrap();
            let i2 = self.select2_idx.unwrap();
            let p1 = &self.points[i1];
            let p2 = &self.points[i2];

            if p1.is_some() && p2.is_some() && Piece::are_friendly(&p1.unwrap(), &p2.unwrap()) {
                self.select1_idx = Some(i2);
                self.select2_idx = None;
            } else {
                self.play();
                self.select1_idx = None;
                self.select2_idx = None;
            }
        } else if self.select1_idx.is_some() && self.select2_idx.is_none() {
            let i1 = self.select1_idx.unwrap();
            let p1 = &self.points[i1];
            if (p1.is_some() && p1.unwrap().color != self.turn) || p1.is_none() {
                self.select1_idx = None;
            }
        }
    }
    pub fn in_bounds(idx: usize) -> bool {
        idx < MAX_BOARD_WIDTH * MAX_BOARD_WIDTH
    }
    pub fn valid_move(&self, from: usize, to: usize) -> bool {
        let p_from = self.points[from].as_ref().unwrap();
        let p_to = self.points[to].as_ref();

        if p_from.color != self.turn {
            return false;
        }

        match p_from.piece_type {
            PieceType::Pawn => {
                (Self::is_forward(from, to, p_from.color) && p_to.is_none())
                    || (p_to.is_some()
                        && Self::is_adjacent_diagnol_forward(from, to, p_from.color)
                        && !Piece::are_friendly(p_from, p_to.unwrap()))
                    || (Self::forward_twice_on_1_or_6(from, to, p_from.color) && p_to.is_none())
            }
            PieceType::Knight => Self::is_knight_hop(from, to),
            PieceType::Bishop => Self::is_diagnol(from, to),
            PieceType::Rook => self.is_slide(from, to),
            PieceType::King => Self::is_adjacent(from, to),
            PieceType::Queen => Self::is_diagnol(from, to) || self.is_slide(from, to),
        }
    }
    fn try_pawn_upgrade(&mut self, pos: usize) {
        match &mut self.points[pos] {
            Some(p) => {
                if p.piece_type != PieceType::Pawn {
                    return;
                }

                if (p.color == PlayerColor::White && Self::row_num(pos) != 7)
                    || (p.color == PlayerColor::Black && Self::row_num(pos) != 0)
                {
                    return;
                }
                p.piece_type = PieceType::Queen
            }
            None => (),
        }
    }
    fn row_num(pos: usize) -> usize {
        pos / 8
    }
    fn col_num(pos: usize) -> usize {
        pos % 8
    }
    fn is_adjacent(from: usize, to: usize) -> bool {
        matches!(from as i32 - to as i32, 8 | -8 | 1 | -1 | -9 | -7 | 9 | 7)
    }
    fn is_adjacent_diagnol_forward(from: usize, to: usize, color: PlayerColor) -> bool {
        match color {
            PlayerColor::White => matches!(from as i32 - to as i32, -7 | -9),
            PlayerColor::Black => matches!(from as i32 - to as i32, 7 | 9),
        }
    }
    fn forward_twice_on_1_or_6(from: usize, to: usize, color: PlayerColor) -> bool {
        if Self::row_num(from) != 1 && Self::row_num(from) != 6 {
            return false;
        }

        let dist = from as i32 - to as i32;

        match color {
            PlayerColor::White => matches!(from as i32 - to as i32, -16),
            PlayerColor::Black => matches!(from as i32 - to as i32, 16),
        }
    }
    fn is_diagnol(from: usize, to: usize) -> bool {
        let dist = from as i32 - to as i32;
        dist % 9 == 0 || dist % 7 == 0
    }
    fn is_knight_hop(from: usize, to: usize) -> bool {
        matches!(
            from as i32 - to as i32,
            17 | 15 | -17 | -15 | -10 | -6 | 10 | 6
        )
    }
    fn is_slide(&self, from: usize, to: usize) -> bool {
        let dist = to as i32 - from as i32;
        let is_row = from / 8 == to / 8;
        let is_col = dist % 8 == 0;

        if !is_col && !is_row {
            return false;
        }

        match is_row {
            // row slide
            true => match dist < 0 {
                // right -> left
                true => {
                    for i in to + 1..from {
                        if self.points[i].is_some() {
                            return false;
                        }
                    }
                }
                // left -> right
                false => {
                    for i in from + 1..to {
                        if self.points[i].is_some() {
                            return false;
                        }
                    }
                }
            },
            // column slide
            false => match dist < 0 {
                // bottom -> top
                true => {
                    let row_depth = from % 8;
                    for i in (to + 1..from).filter(|n| n % 8 == row_depth) {
                        if self.points[i].is_some() {
                            return false;
                        }
                    }
                }
                // top -> bottom
                false => {
                    let row_depth = from % 8;
                    for i in (from + 1..to).filter(|n| n % 8 == row_depth) {
                        if self.points[i].is_some() {
                            return false;
                        }
                    }
                }
            },
        }

        true
    }
    fn is_forward(from: usize, to: usize, color: PlayerColor) -> bool {
        match color {
            PlayerColor::White => from + 8 == to,
            PlayerColor::Black => from - 8 == to,
        }
    }
    fn are_friendly(&self, i1: usize, i2: usize) -> bool {
        if let (Some(p1), Some(p2)) = (self.points[i1], self.points[i2]) {
            return Piece::are_friendly(&p1, &p2);
        }

        false
    }
}

impl Default for BoardState {
    fn default() -> Self {
        BoardState {
            turn: PlayerColor::White,
            select1_idx: None,
            select2_idx: None,
            points: [
                Some(Piece::new(PieceType::Rook, PlayerColor::White)),
                Some(Piece::new(PieceType::Knight, PlayerColor::White)),
                Some(Piece::new(PieceType::Bishop, PlayerColor::White)),
                Some(Piece::new(PieceType::King, PlayerColor::White)),
                Some(Piece::new(PieceType::Queen, PlayerColor::White)),
                Some(Piece::new(PieceType::Bishop, PlayerColor::White)),
                Some(Piece::new(PieceType::Knight, PlayerColor::White)),
                Some(Piece::new(PieceType::Rook, PlayerColor::White)),
                Some(Piece::new(PieceType::Pawn, PlayerColor::White)),
                Some(Piece::new(PieceType::Pawn, PlayerColor::White)),
                Some(Piece::new(PieceType::Pawn, PlayerColor::White)),
                Some(Piece::new(PieceType::Pawn, PlayerColor::White)),
                Some(Piece::new(PieceType::Pawn, PlayerColor::White)),
                Some(Piece::new(PieceType::Pawn, PlayerColor::White)),
                Some(Piece::new(PieceType::Pawn, PlayerColor::White)),
                Some(Piece::new(PieceType::Pawn, PlayerColor::White)),
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
                Some(Piece::new(PieceType::Pawn, PlayerColor::Black)),
                Some(Piece::new(PieceType::Pawn, PlayerColor::Black)),
                Some(Piece::new(PieceType::Pawn, PlayerColor::Black)),
                Some(Piece::new(PieceType::Pawn, PlayerColor::Black)),
                Some(Piece::new(PieceType::Pawn, PlayerColor::Black)),
                Some(Piece::new(PieceType::Pawn, PlayerColor::Black)),
                Some(Piece::new(PieceType::Pawn, PlayerColor::Black)),
                Some(Piece::new(PieceType::Pawn, PlayerColor::Black)),
                Some(Piece::new(PieceType::Rook, PlayerColor::Black)),
                Some(Piece::new(PieceType::Knight, PlayerColor::Black)),
                Some(Piece::new(PieceType::Bishop, PlayerColor::Black)),
                Some(Piece::new(PieceType::King, PlayerColor::Black)),
                Some(Piece::new(PieceType::Queen, PlayerColor::Black)),
                Some(Piece::new(PieceType::Bishop, PlayerColor::Black)),
                Some(Piece::new(PieceType::Knight, PlayerColor::Black)),
                Some(Piece::new(PieceType::Rook, PlayerColor::Black)),
            ],
        }
    }
}
