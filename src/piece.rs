//use std::fmt;
use std::str::FromStr;
//extern crate bounded_integer;
//use bounded_integer::BoundedU8;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PieceColor {
  Black, White,
}

trait ToPieceColor {
  fn to_piece_color(self) -> PieceColor;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PieceKind {
  Pawn, 
  Knight, Bishop, Rook,
  King, Queen,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Piece {
  pub color: PieceColor,
  pub kind: PieceKind,
}