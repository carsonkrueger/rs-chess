use std::rc::Rc;

use crate::library::piece::Piece;
use crate::library::player::{PlayerColor, PlayerState};
use crate::Board;
use gloo::console::log;
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
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
        } else if p2.is_some() && Piece::are_friendly(&p1.unwrap(), &p2.unwrap()) {
            return Err(MoveError::InvalidMove);
        }

        self.points[self.select1_idx.unwrap()] = None;
        self.points[self.select2_idx.unwrap()] = p1.clone();

        // let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        // let file = BufReader::new(File::open("sound/move-self.mp3").unwrap());
        // let source = Decoder::new(file).unwrap();
        // stream_handle.play_raw(source.convert_samples());
        // std::thread::sleep(std::time::Duration::from_secs(1));

        self.try_pawn_upgrade(self.select2_idx.unwrap()); // Upgrades pawns on other side of board

        self.turn = match self.turn {
            PlayerColor::WHITE => PlayerColor::BLACK,
            PlayerColor::BLACK => PlayerColor::WHITE,
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

        if p_from.color != self.turn {
            return false;
        }

        match p_from.piece_type {
            PieceType::PAWN => {
                (Self::is_forward(from, to, p_from.color) && p_to.is_none())
                    || (p_to.is_some()
                        && Self::is_adjacent_diagnol_forward(from, to, p_from.color)
                        && !Piece::are_friendly(p_from, p_to.unwrap()))
                    || (Self::forward_twice_on_1_or_6(from, to, p_from.color) && p_to.is_none())
            }
            PieceType::KNIGHT => Self::is_knight_hop(from, to),
            PieceType::BISHOP => Self::is_diagnol(from, to),
            PieceType::ROOK => self.is_slide(from, to),
            PieceType::KING => Self::is_adjacent(from, to),
            PieceType::QUEEN => Self::is_diagnol(from, to) || self.is_slide(from, to),
        }
    }
    fn try_pawn_upgrade(&mut self, pos: usize) {
        match &mut self.points[pos] {
            Some(p) => {
                if p.piece_type != PieceType::PAWN {
                    return;
                }

                if p.color == PlayerColor::WHITE && Self::row_num(pos) != 7 {
                    return;
                } else if p.color == PlayerColor::BLACK && Self::row_num(pos) != 0 {
                    return;
                }

                p.piece_type = PieceType::QUEEN
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
                -9 => true,
                -7 => true,
                _ => false,
            },
            PlayerColor::BLACK => match from as i32 - to as i32 {
                7 => true,
                9 => true,
                _ => false,
            },
        }
    }
    fn forward_twice_on_1_or_6(from: usize, to: usize, color: PlayerColor) -> bool {
        if Self::row_num(from) != 1 && Self::row_num(from) != 6 {
            return false;
        }

        let dist = from as i32 - to as i32;

        match color {
            PlayerColor::WHITE => match from as i32 - to as i32 {
                -16 => true,
                _ => false,
            },
            PlayerColor::BLACK => match from as i32 - to as i32 {
                16 => true,
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
            PlayerColor::WHITE => from + 8 == to,
            PlayerColor::BLACK => from - 8 == to,
        }
    }
    fn are_friendly(&self, i1: usize, i2: usize) -> bool {
        let p1 = self.points[i1];
        let p2 = self.points[i2];

        if p1.is_some() && p2.is_some() {
            return Piece::are_friendly(&p1.unwrap(), &p2.unwrap());
        }

        false
    }
}

impl Default for BoardState {
    fn default() -> Self {
        BoardState {
            turn: PlayerColor::WHITE,
            select1_idx: None,
            select2_idx: None,
            points: [
                Some(Piece::new(PieceType::ROOK, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::KNIGHT, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::BISHOP, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::KING, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::QUEEN, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::BISHOP, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::KNIGHT, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::ROOK, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::PAWN, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::PAWN, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::PAWN, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::PAWN, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::PAWN, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::PAWN, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::PAWN, PlayerColor::WHITE)),
                Some(Piece::new(PieceType::PAWN, PlayerColor::WHITE)),
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
                Some(Piece::new(PieceType::PAWN, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::PAWN, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::PAWN, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::PAWN, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::PAWN, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::PAWN, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::PAWN, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::PAWN, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::ROOK, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::KNIGHT, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::BISHOP, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::KING, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::QUEEN, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::BISHOP, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::KNIGHT, PlayerColor::BLACK)),
                Some(Piece::new(PieceType::ROOK, PlayerColor::BLACK)),
            ],
        }
    }
}
