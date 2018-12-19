use std::rc::Rc;
use super::helpers;
use super::Board;

#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Square {
    pub piece: Option<Piece>,
    pub is_edge: bool,
}

impl Square {
    pub fn new(piece: Piece) -> Square {
        Square { piece: Some(piece), is_edge: false }
    }
}

#[derive(Copy, Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Color {
    White, Black
}

#[derive(Copy, Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Piece {
    Pawn(Color), Knight(Color), Bishop(Color), Rook(Color), Queen(Color), King(Color)
}

impl Piece {
    pub fn get_moves(&self, board_index: usize, board: Rc<Board>) -> Vec<Move> {
        match self {
            &Piece::Pawn(Color::White) => valid_moves::get_white_pawn_moves(board_index, board),
            &Piece::Pawn(Color::Black) => valid_moves::get_black_pawn_moves(board_index, board),
            &Piece::Knight(Color::White) => valid_moves::get_white_knight_moves(board_index, board),
            &Piece::Knight(Color::Black) => valid_moves::get_black_knight_moves(board_index, board),
            &Piece::Bishop(Color::White) => valid_moves::get_white_bishop_moves(board_index, board),
            &Piece::Bishop(Color::Black) => valid_moves::get_black_bishop_moves(board_index, board),
            &Piece::Rook(Color::White) => valid_moves::get_white_rook_moves(board_index, board),
            &Piece::Rook(Color::Black) => valid_moves::get_black_rook_moves(board_index, board),
            &Piece::Queen(Color::White) => valid_moves::get_white_queen_moves(board_index, board),
            &Piece::Queen(Color::Black) => valid_moves::get_black_queen_moves(board_index, board),
            &Piece::King(Color::White) => valid_moves::get_white_king_moves(board_index, board),
            &Piece::King(Color::Black) => valid_moves::get_black_king_moves(board_index, board),
        }
    }
}

#[derive(Debug)]
pub struct Move {
    pub from: usize,
    pub to: usize,
}

impl Move {

    pub fn new(from: u8, to: u8) -> Move {
        Move { from: from as usize, to: to as usize }
    }

    pub fn from_chess_move(chess_move: (String, String)) -> Move {
        let from = helpers::square_to_index(chess_move.0);
        let to = helpers::square_to_index(chess_move.1);

        Move { from, to }
    }

    pub fn to_chess_move(&self) -> (String, String) {
        let from = helpers::index_to_square(self.from);
        let to = helpers::index_to_square(self.to);

        (from, to)
    }
}

#[derive(Debug)]
pub struct Turn {
    pub color: Color
}

impl Turn {
    pub fn toggle(&mut self) {
        self.color = match self.color {
            Color::White => Color::Black,
            _ => Color::White,
        }
    }
}

mod valid_moves {
    use super::{ Board, Rc, Move };

    pub fn get_white_pawn_moves(board_index: usize, board: Rc<Board>) -> Vec<Move> {
        Vec::new()
    }

    pub fn get_black_pawn_moves(board_index: usize, board: Rc<Board>) -> Vec<Move> {
        Vec::new()
    }

    pub fn get_white_knight_moves(board_index: usize, board: Rc<Board>) -> Vec<Move> {
        Vec::new()
    }

    pub fn get_black_knight_moves(board_index: usize, board: Rc<Board>) -> Vec<Move> {
        Vec::new()
    }

    pub fn get_white_bishop_moves(board_index: usize, board: Rc<Board>) -> Vec<Move> {
        Vec::new()
    }

    pub fn get_black_bishop_moves(board_index: usize, board: Rc<Board>) -> Vec<Move> {
        Vec::new()
    }

    pub fn get_white_rook_moves(board_index: usize, board: Rc<Board>) -> Vec<Move> {
        Vec::new()
    }

    pub fn get_black_rook_moves(board_index: usize, board: Rc<Board>) -> Vec<Move> {
        Vec::new()
    }

    pub fn get_white_queen_moves(board_index: usize, board: Rc<Board>) -> Vec<Move> {
        Vec::new()
    }

    pub fn get_black_queen_moves(board_index: usize, board: Rc<Board>) -> Vec<Move> {
        Vec::new()
    }

    pub fn get_white_king_moves(board_index: usize, board: Rc<Board>) -> Vec<Move> {
        Vec::new()
    }

    pub fn get_black_king_moves(board_index: usize, board: Rc<Board>) -> Vec<Move> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {

}