// Implementation of Chess Rules
// We consider chess moves as transition functions on the game state
// Moves can either be standard moves, castling, promotion (of pawn to queen)

use std::fmt;
use crate::piece::*;
use crate::board::*;

type Player = PieceColor;

pub struct ChessState {
  board: Board,
  black_can_castle: bool,
  white_can_castle: bool,
}

pub enum GameState {
  OnGoing(ChessState),
  Check(ChessState, Player),
  CheckMate(ChessState, Player),
  StaleMate(ChessState),
}

impl GameState {
  pub fn promote(self, pos: (u8, u8)) -> GameState {
    match self {
      GameState::OnGoing(chess) => {
        if pos.0 == 7 || pos.1 == 7 {
          GameState::OnGoing(chess)
        } else { panic!("invalid move") }
      },
      _ => { panic!("invalid move") }
    }
  }

  pub fn move_to(self, from: (u8, u8), to: (u8, u8)) -> GameState {
    self
  }

  pub fn castle(self, rook_pos: (u8,u8), king_pos: (u8, u8)) -> GameState {
    self
  }
}