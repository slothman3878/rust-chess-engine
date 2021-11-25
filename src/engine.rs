// Implementation of Chess Rules
// We consider chess moves as transition functions on the game state
// Moves can either be standard moves, castling, promotion (of pawn to queen)

use std::fmt;
use crate::piece::*;
use crate::board::*;

type Player = PieceColor;

impl Player {
  pub fn next(self) -> Player {
    match self {
      PieceColor::White => PieceColor::Black,
      PieceColor::Black => PieceColor::White,
    }
  }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct BoardState {
  board: Board,
  black_can_castle: bool,
  white_can_castle: bool,
  black_king: (u8, u8),
  white_king: (u8, u8),
}

impl BoardState {
  pub fn new() -> BoardState {
    BoardState {
      board: Board::default(),
      black_can_castle: true,
      white_can_castle: true,
      black_king: (0,3),
      white_king: (7,3),
    }
  }

  pub fn place(self, piece: Piece, pos: (u8,u8)) -> BoardState {
    let mut new_state = self.clone();
    new_state.board.grid[pos.0 as usize][pos.1 as usize] = Square::Full(piece);
    new_state
  }

  pub fn remove(self, pos: (u8,u8)) -> BoardState {
    let mut new_state = self.clone();
    new_state.board.grid[pos.0 as usize][pos.1 as usize] = Square::Empty;
    new_state
  }

  pub fn evaluate(self) -> GameState {
    
  }
}

pub enum GameState {
  OnGoing{turn: Player, state: BoardState},
  Check{state: BoardState, by: Player},
  CheckMate{state: BoardState, by: Player},
  StaleMate{state: BoardState},
}

impl GameState {
  pub fn new() -> GameState {
    GameState::OnGoing{
      turn: PieceColor::White,
      state: BoardState::new(),
    }
  }

  pub fn promote(self, pos: (u8, u8)) -> GameState {
    match self {
      GameState::OnGoing{turn, state} => {
        if pos.0 == 7 || pos.1 == 7 {
          match state.board.grid[pos.0 as usize][pos.1 as usize] {
            Square::Full(piece) => {
              if(piece.color == turn && piece.kind == PieceKind::Pawn) {
                let new_state = state.place(Piece{color: turn, kind: PieceKind::Queen}, pos);
                // check if new_state is valid
                // check if new_state results in a check
                // check if new_state results in a check_mate
                GameState::OnGoing {
                  turn: turn.next(),
                  state: state.place(Piece{color: turn, kind: PieceKind::Queen}, pos),
                }
              } else { panic!("invalid move") }
            },
            Empty => { panic!("invalid move") } 
          }
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