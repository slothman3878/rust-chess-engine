use crate::piece::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Square {
  Empty,
  Full(Piece)
}

pub type Grid = [[Square;8];8];

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Board {
  grid: Grid,
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

  pub fn square(self, row: u8, col: u8) -> Square {
    self.grid[row as usize][col as usize]
  }

  pub fn replace(self, piece: Piece, row: u8, col: u8) -> Board {
    let mut new_board = self.clone();
    new_board.grid[row as usize][col as usize] = Square::Full(piece);
    new_board
  }

  pub fn remove(self, row: u8, col: u8) -> Board {
    let mut new_board = self.clone();
    new_board.grid[row as usize][col as usize] = Square::Empty;
    new_board
  }

  pub fn switch(self, pos1: (u8,u8), pos2: (u8,u8)) -> Board {
    let mut new_board = self.clone();
    new_board.grid[pos1.0 as usize][pos1.1 as usize] = self.square(pos2.0,pos2.1);
    new_board.grid[pos2.0 as usize][pos2.1 as usize] = self.square(pos1.0,pos1.1);
    new_board
  }
}