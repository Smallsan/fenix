use crate::chess_board::{board::Board, coordinates::Coordinates};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn reverse(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CastlingRights {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}

impl Default for CastlingRights {
    fn default() -> Self {
        Self {
            white_kingside: true,
            white_queenside: true,
            black_kingside: true,
            black_queenside: true,
        }
    }
}

pub trait ChessPiece {
    fn piece_type(&self) -> PieceType;
    fn color(&self) -> Color;
    fn coordinates(&self) -> Coordinates;
    fn change_color(&mut self, color: Color);
    fn change_coordinates(&mut self, coordinates: Coordinates);
    fn move_to(&self, destination: Coordinates, board: &mut Board) -> Result<(), &'static str>;
    fn is_valid_move(&mut self, coordinates: Coordinates, board: &Board) -> bool;
}
