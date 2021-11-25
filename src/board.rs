use std::fmt;
use crate::piece::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Square {
  Empty,
  Full(Piece)
}

pub type Grid = [[Square;8];8];

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Board {
  pub grid: Grid,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum BoardState {
  Check(PieceColor), CheckMate(PieceColor), None
}

const DEFAULT_BOARD_STATE: Grid = [
  [
    Square::Full(Piece{color: PieceColor::Black, kind: PieceKind::Rook}),
    Square::Full(Piece{color: PieceColor::Black, kind: PieceKind::Bishop}),
    Square::Full(Piece{color: PieceColor::Black, kind: PieceKind::Knight}),
    Square::Full(Piece{color: PieceColor::Black, kind: PieceKind::King}),
    Square::Full(Piece{color: PieceColor::Black, kind: PieceKind::Queen}),
    Square::Full(Piece{color: PieceColor::Black, kind: PieceKind::Knight}),
    Square::Full(Piece{color: PieceColor::Black, kind: PieceKind::Bishop}),
    Square::Full(Piece{color: PieceColor::Black, kind: PieceKind::Rook})
  ],
  [
    Square::Full(Piece{color: PieceColor::Black, kind: PieceKind::Pawn}),
    Square::Full(Piece{color: PieceColor::Black, kind: PieceKind::Pawn}),
    Square::Full(Piece{color: PieceColor::Black, kind: PieceKind::Pawn}),
    Square::Full(Piece{color: PieceColor::Black, kind: PieceKind::Pawn}),
    Square::Full(Piece{color: PieceColor::Black, kind: PieceKind::Pawn}),
    Square::Full(Piece{color: PieceColor::Black, kind: PieceKind::Pawn}),
    Square::Full(Piece{color: PieceColor::Black, kind: PieceKind::Pawn}),
    Square::Full(Piece{color: PieceColor::Black, kind: PieceKind::Pawn}),
  ],
  [Square::Empty; 8],
  [Square::Empty; 8],
  [Square::Empty; 8],
  [Square::Empty; 8],
  [
    Square::Full(Piece{color: PieceColor::White, kind: PieceKind::Pawn}),
    Square::Full(Piece{color: PieceColor::White, kind: PieceKind::Pawn}),
    Square::Full(Piece{color: PieceColor::White, kind: PieceKind::Pawn}),
    Square::Full(Piece{color: PieceColor::White, kind: PieceKind::Pawn}),
    Square::Full(Piece{color: PieceColor::White, kind: PieceKind::Pawn}),
    Square::Full(Piece{color: PieceColor::White, kind: PieceKind::Pawn}),
    Square::Full(Piece{color: PieceColor::White, kind: PieceKind::Pawn}),
    Square::Full(Piece{color: PieceColor::White, kind: PieceKind::Pawn}),
  ],
  [
    Square::Full(Piece{color: PieceColor::White, kind: PieceKind::Rook}),
    Square::Full(Piece{color: PieceColor::White, kind: PieceKind::Bishop}),
    Square::Full(Piece{color: PieceColor::White, kind: PieceKind::Knight}),
    Square::Full(Piece{color: PieceColor::White, kind: PieceKind::King}),
    Square::Full(Piece{color: PieceColor::White, kind: PieceKind::Queen}),
    Square::Full(Piece{color: PieceColor::White, kind: PieceKind::Knight}),
    Square::Full(Piece{color: PieceColor::White, kind: PieceKind::Bishop}),
    Square::Full(Piece{color: PieceColor::White, kind: PieceKind::Rook})
  ],
];

impl Board {
  pub fn default() -> Board {
    Board {
      grid: DEFAULT_BOARD_STATE
    }
  }

  pub fn is_check(self) -> BoardState {
    for row in self.grid.iter() {
      for square in row.iter() {
        match square {
          Square::Full(piece) => {
            
          },
          Square::Empty => {}
        }
      }
    }
    BoardState::None
  }
}