use crate::chess::{board::{board::{Board, BoardType}, coordinates::Coordinates}, piece::piece::{Color, PieceType}};




#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Move {
    piece_type: PieceType,
    from: Coordinates,
    to: Coordinates,
}

impl Move {
    pub fn new(from: Coordinates, to: Coordinates, piece_type: PieceType) -> Self {
        Self {
            from,
            to,
            piece_type,
        }
    }

    pub fn from(&self) -> Coordinates {
        self.from
    }

    pub fn to(&self) -> Coordinates {
        self.to
    }

    pub fn piece_type(&self) -> PieceType {
        self.piece_type
    }
}


impl Board {
    
        /// Generates all possible moves for a chess color in the board.
        /// Doesn't check for pins and check situations yet
        /// Can't generate En passants and castles yet
        pub fn generate_moves(&self, color: Color) -> Vec<Move> {
            let mut moves = Vec::new();
    
            match &self.board_type {
                BoardType::OneDimensional(board) => {
                    for i in 1..=64 {
                        if let Some(piece) = board[i - 1] {
                            if piece.color() == color {
                                for x_to in 1..=8 {
                                    for y_to in 1..=8 {
                                        let destination = Coordinates::new(x_to, y_to);
                                        let piece_type = piece.piece_type();
                                        if piece.is_valid_move(destination, self) {
                                            moves.push(Move::new(
                                                piece.coordinates(),
                                                destination,
                                                piece_type,
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                BoardType::TwoDimensional(board) => {
                    for x_from in 1..=8 {
                        for y_from in 1..=8 {
                            if let Some(piece) = board[x_from - 1][y_from - 1] {
                                if piece.color() == color {
                                    for x_to in 1..=8 {
                                        for y_to in 1..=8 {
                                            let piece_type = piece.piece_type();
                                            let destination = Coordinates::new(x_to, y_to);
                                            if piece.is_valid_move(destination, self) {
                                                moves.push(Move::new(
                                                    piece.coordinates(),
                                                    destination,
                                                    piece_type,
                                                ));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
    
            moves
        }
}