// Implementation of Chess Rules
// We consider chess moves as transition functions on the game state
// Moves can either be standard moves, castling, promotion (of pawn to queen)

use std::fmt;
use crate::piece::*;
use crate::board::*;
use crate::util::LeftOrRight;

type Player = PieceColor;

impl Player {
  fn next(self) -> Player {
    match self {
      PieceColor::White => PieceColor::Black,
      PieceColor::Black => PieceColor::White,
    }
  }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct BoardState {
  board: Board,
  //check: Option<Player>, // player checks opponent
  black_can_castle: bool,
  white_can_castle: bool,
  black_king: (u8, u8),
  white_king: (u8, u8),
}

impl BoardState {
  fn new() -> BoardState {
    BoardState {
      board: Board::default(),
      black_can_castle: true,
      white_can_castle: true,
      black_king: (0,3),
      white_king: (7,3),
    }
  }

  pub fn is_check(self) -> Option<Player> {
    None
  }
}

#[derive(PartialEq, Debug)]
pub struct ChessMove {
  player: Player,
  kind: ChessMoveKind,
}

#[derive(PartialEq, Debug)]
pub enum ChessMoveKind {
  Standard{from: (u8, u8), to: (u8, u8)},
  Promote{at: (u8, u8)},
  Castle(LeftOrRight),
}

#[derive(PartialEq, Debug)]
pub enum ChessMoveError {
  EmptySquare(ChessMove),
  InvalidPiece(ChessMove),
  BlockedPath(ChessMove),
  CastleRestricted(ChessMove),
  ExposedKing(ChessMove),
}

#[derive(PartialEq, Debug)]
pub enum GameState {
  OnGoing{turn: Player, state: BoardState},
  Check{turn: Player, state: BoardState},
  Promotion{turn: Player, state: BoardState},
  CheckMate{state: BoardState, by: Player},
  StaleMate{state: BoardState},
}

#[derive(PartialEq, Debug)]
pub enum GameStateError {
  WrongTurn(Player),
  WrongState,
  InvalidMove(ChessMoveError),
}

impl ChessMove {
  // is the chess move valid for a given board state
  // if not results in error
  // if yes, results in game state
  pub fn is_valid(self, board_state: BoardState) -> Result<GameState,ChessMoveError> {
    let new_board_state = match self.kind {
      // promotion move
      ChessMoveKind::Promote{at} => {
        let square = board_state.board.square(at.0, at.1);
        match square {
          Square::Full(piece) => {
            if piece.color != self.player { return Err(ChessMoveError::InvalidPiece(self)); }
            match piece.kind {
              PieceKind::Pawn => {
                BoardState {
                  board: board_state.board.replace(
                    Piece{
                      kind: PieceKind::Queen,
                      color: self.player,
                    }, at.0, at.1),
                  black_can_castle: board_state.black_can_castle,
                  white_can_castle: board_state.white_can_castle,
                  black_king: board_state.black_king,
                  white_king: board_state.white_king,
                }
              },
              _ => { return Err(ChessMoveError::InvalidPiece(self)); }
            }
          },
          Square::Empty => {
            return Err(ChessMoveError::EmptySquare(self));
          }
        }
      },
      // castling
      ChessMoveKind::Castle(rook_pos) => {
        match self.player {
          PieceColor::Black => {
            // can the player still castle
            if !board_state.black_can_castle { return Err(ChessMoveError::CastleRestricted(self)); }
            // is the king in the right position
            match board_state.board.square(0,3) {
              Square::Full(piece) => {
                if piece.kind == PieceKind::King && piece.color == self.player {
                  // king is in right position
                  // now see if the given rook is correctly positioned
                  match rook_pos {
                    Left => {
                      match board_state.board.square(0,7) {
                        Square::Full(rook) => {
                          if rook.color == self.player && rook.kind == PieceKind::Rook {         
                            BoardState {
                              board: ..,
                              black_can_castle: !board_state.black_can_castle,
                              white_can_castle: board_state.white_can_castle,
                              black_king: (),
                              white_king: board_state.white_king,
                            }
                          } else {
                            return Err(ChessMoveError::InvalidPiece(self))
                          }
                        },
                        Square::Empty => {
                          return Err(ChessMoveError::EmptySquare(self));
                        }
                      }
                    },
                    Right => {
                      match board_state.board.square(0,0) {
                        Square::Full(rook) => {
                          if rook.color == self.player && rook.kind == PieceKind::Rook {         
                            BoardState {
                              board: board_state.board.switch((0,0),(0,3)),
                              black_can_castle: board_state.black_can_castle,
                              white_can_castle: board_state.white_can_castle,
                              black_king: board_state.black_king,
                              white_king: board_state.white_king,
                            }
                          } else {
                            return Err(ChessMoveError::InvalidPiece(self))
                          }
                        },
                        Square::Empty => {
                          return Err(ChessMoveError::EmptySquare(self));
                        }
                      }
                    }
                  }
                }
                else { return Err(ChessMoveError::InvalidPiece(self)); }
              },
              Square::Empty => { return Err(ChessMoveError::EmptySquare(self)); }
            }
          },
          PieceColor::White => {
            // can the player still castle
            if !board_state.white_can_castle { return Err(ChessMoveError::CastleRestricted(self)); }
            // is the king in the right position
            match board_state.board.square(7,3) {
              Square::Full(piece) => {
                if piece.kind == PieceKind::King && piece.color == self.player {
                  // king is in right position
                  // now see if the given rook is correctly positioned
                  match rook_pos {
                    Left => {
                      match board_state.board.square(7,0) {
                        Square::Full(rook) => {
                          if rook.color == self.player && rook.kind == PieceKind::Rook {         
                            BoardState {
                              board: board_state.board.switch((7,0),(7,3)),
                              black_can_castle: board_state.black_can_castle,
                              white_can_castle: board_state.white_can_castle,
                              black_king: board_state.black_king,
                              white_king: board_state.white_king,
                            }
                          } else {
                            return Err(ChessMoveError::InvalidPiece(self))
                          }
                        },
                        Square::Empty => {
                          return Err(ChessMoveError::EmptySquare(self));
                        }
                      }
                    },
                    Right => {
                      match board_state.board.square(7,7) {
                        Square::Full(rook) => {
                          if rook.color == self.player && rook.kind == PieceKind::Rook {         
                            BoardState {
                              board: board_state.board.switch((7,7),(7,3)),
                              black_can_castle: board_state.black_can_castle,
                              white_can_castle: board_state.white_can_castle,
                              black_king: board_state.black_king,
                              white_king: board_state.white_king,
                            }
                          } else {
                            return Err(ChessMoveError::InvalidPiece(self))
                          }
                        },
                        Square::Empty => {
                          return Err(ChessMoveError::EmptySquare(self));
                        }
                      }
                    }
                  }
                }
                else { return Err(ChessMoveError::InvalidPiece(self)); }
              },
              Square::Empty => { return Err(ChessMoveError::EmptySquare(self)); }
            }
          }
        }
      },
      _ => { return Err(ChessMoveError::BlockedPath(self)); }
    };
    Ok(GameState::OnGoing{
      turn: self.player.next(),
      state: new_board_state
    })
  }
}

impl GameState {
  pub fn new() -> GameState {
    GameState::OnGoing{
      turn: PieceColor::White,
      state: BoardState::new(),
    }
  }

  pub fn chess_move(self, mv: ChessMove) -> Result<GameState,GameStateError> {
    match self {
      GameState::Promotion{turn, state} => {
        if turn == mv.player {
          match mv.is_valid(state) {
            Ok(new_state) => { Ok(new_state) },
            Err(err) => { Err(GameStateError::InvalidMove(err)) }
          }
        } else { Err(GameStateError::WrongTurn(turn)) }
      },
      _ => { Err(GameStateError::WrongState) }
    }
  }
}