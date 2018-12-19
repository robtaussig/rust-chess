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
pub enum PieceType {
    Pawn, Knight, Bishop, Rook, Queen, King
}

#[derive(Copy, Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Piece {
        Piece { piece_type, color }
    }

    pub fn get_moves(&self, board_index: usize, board: Rc<Board>) -> Vec<Move> {
        match self.color {
            Color::White => {
                match self.piece_type {
                    PieceType::Pawn => valid_moves::get_white_pawn_moves(board_index as i8, board),
                    PieceType::Knight => valid_moves::get_white_knight_moves(board_index as i8, board),
                    PieceType::Bishop => valid_moves::get_white_bishop_moves(board_index as i8, board),
                    PieceType::Rook => valid_moves::get_white_rook_moves(board_index as i8, board),
                    PieceType::Queen => valid_moves::get_white_queen_moves(board_index as i8, board),
                    PieceType::King => valid_moves::get_white_king_moves(board_index as i8, board),
                }
            },
            Color::Black => {
                match self.piece_type {
                    PieceType::Pawn => valid_moves::get_black_pawn_moves(board_index as i8, board),
                    PieceType::Knight => valid_moves::get_black_knight_moves(board_index as i8, board),
                    PieceType::Bishop => valid_moves::get_black_bishop_moves(board_index as i8, board),
                    PieceType::Rook => valid_moves::get_black_rook_moves(board_index as i8, board),
                    PieceType::Queen => valid_moves::get_black_queen_moves(board_index as i8, board),
                    PieceType::King => valid_moves::get_black_king_moves(board_index as i8, board),
                }
            },
        }
    }
}

#[derive(Debug)]
pub struct Move {
    pub from: usize,
    pub to: usize,
}

impl Move {

    pub fn new(from: usize, to: usize) -> Move {
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

pub mod valid_moves {
    use super::{ Board, Rc, Move, Color };

    #[allow(unused)]
    const BISHOP_MOVE_DIRECTIONS: [i8; 4] = [9, 11, -9, -11];
    const KNIGHT_STEPPING_MOVES: [i8; 8] = [-12, -21, -19, -8, 12, 21, 19, 8];
    #[allow(unused)]
    const KING_QUEEN_MOVE_DIRECTIONS: [i8; 8] = [-1, -11, -10, -9, 1, 11, 10, 9];
    #[allow(unused)]
    const ROOK_MOVE_DIRECTIONS: [i8; 4] = [-1, 1, -10, 10];
    #[allow(unused)]
    const WHITE_PAWN_MOVE_DIRECTIONS: [i8; 1] = [-10];
    #[allow(unused)]
    const BLACK_PAWN_MOVE_DIRECTIONS: [i8; 1] = [10];

    pub fn is_out_of_bounds(board_index: i8) -> bool {
        board_index < 11 || board_index > 88 || board_index % 10 == 0 || board_index % 10 == 9
    }

    #[allow(unused)]
    pub fn get_white_pawn_moves(board_index: i8, board: Rc<Board>) -> Vec<Move> {
        
        Vec::new()
    }

    #[allow(unused)]
    pub fn get_black_pawn_moves(board_index: i8, board: Rc<Board>) -> Vec<Move> {
        Vec::new()
    }

    pub fn get_white_knight_moves(board_index: i8, board: Rc<Board>) -> Vec<Move> {
        let legal_moves: Vec<Move> = KNIGHT_STEPPING_MOVES.into_iter()
            .map(|step| board_index + step)
            .filter(|to| {
                if is_out_of_bounds(*to) { return false; }
                match board.get_piece_at(*to as usize) {
                    Some(p) => p.color == Color::Black,
                    None => true
                }
            })
            .map(|to| Move::new(board_index as usize, to as usize))
            .collect();

        legal_moves
    }

    pub fn get_black_knight_moves(board_index: i8, board: Rc<Board>) -> Vec<Move> {
        let legal_moves: Vec<Move> = KNIGHT_STEPPING_MOVES.into_iter()
            .map(|step| board_index + step)
            .filter(|to| {
                if is_out_of_bounds(*to) { return false; }
                match board.get_piece_at(*to as usize) {
                    Some(p) => p.color == Color::White,
                    None => true
                }
            })
            .map(|to| Move::new(board_index as usize, to as usize))
            .collect();

        legal_moves
    }

    #[allow(unused)]
    pub fn get_white_bishop_moves(board_index: i8, board: Rc<Board>) -> Vec<Move> {
        Vec::new()
    }

    #[allow(unused)]
    pub fn get_black_bishop_moves(board_index: i8, board: Rc<Board>) -> Vec<Move> {
        Vec::new()
    }

    #[allow(unused)]
    pub fn get_white_rook_moves(board_index: i8, board: Rc<Board>) -> Vec<Move> {
        Vec::new()
    }

    #[allow(unused)]
    pub fn get_black_rook_moves(board_index: i8, board: Rc<Board>) -> Vec<Move> {
        Vec::new()
    }

    #[allow(unused)]
    pub fn get_white_queen_moves(board_index: i8, board: Rc<Board>) -> Vec<Move> {
        Vec::new()
    }

    #[allow(unused)]
    pub fn get_black_queen_moves(board_index: i8, board: Rc<Board>) -> Vec<Move> {
        Vec::new()
    }

    #[allow(unused)]
    pub fn get_white_king_moves(board_index: i8, board: Rc<Board>) -> Vec<Move> {
        Vec::new()
    }

    #[allow(unused)]
    pub fn get_black_king_moves(board_index: i8, board: Rc<Board>) -> Vec<Move> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod valid_moves {
        use super::*;

        mod get_white_pawn_moves {            

            #[test]
            fn it_should_return_a_list_of_moves_from_startin_position() {
                assert!(true);       
            }
        }

        mod get_black_pawn_moves {            

            #[test]
            fn it_should_return_a_list_of_moves_from_startin_position() {
                assert!(true);       
            }
        }

        mod get_white_knight_moves {
            use super::*; 
    
            #[test]
            fn it_should_return_a_list_of_moves_from_startin_position() {
                let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
                let board: Board = helpers::generate_board(board_string);
                let knight = board.get_piece_at(82 as usize).unwrap();
                let legal_moves: Vec<Move> = knight.get_moves(82 as usize, Rc::new(board));
                assert_eq!(legal_moves.len(), 2);
            }
        }

        mod get_black_knight_moves {            
            use super::*;

            #[test]
            fn it_should_return_a_list_of_moves_from_startin_position() {
                let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
                let board: Board = helpers::generate_board(board_string);
                let knight = board.get_piece_at(12 as usize).unwrap();
                let legal_moves: Vec<Move> = knight.get_moves(12 as usize, Rc::new(board));
                assert_eq!(legal_moves.len(), 2);
            }
        }

        mod get_white_bishop_moves {            

            #[test]
            fn it_should_return_a_list_of_moves_from_startin_position() {
                assert!(true);       
            }
        }

        mod get_black_bishop_moves {            

            #[test]
            fn it_should_return_a_list_of_moves_from_startin_position() {
                assert!(true);       
            }
        }

        mod get_white_rook_moves {            

            #[test]
            fn it_should_return_a_list_of_moves_from_startin_position() {
                assert!(true);       
            }
        }

        mod get_black_rook_moves {            

            #[test]
            fn it_should_return_a_list_of_moves_from_startin_position() {
                assert!(true);       
            }
        }

        mod get_white_queen_moves {            

            #[test]
            fn it_should_return_a_list_of_moves_from_startin_position() {
                assert!(true);       
            }
        }

        mod get_black_queen_moves {            

            #[test]
            fn it_should_return_a_list_of_moves_from_startin_position() {
                assert!(true);       
            }
        }

        mod get_white_king_moves {            

            #[test]
            fn it_should_return_a_list_of_moves_from_startin_position() {
                assert!(true);       
            }
        }

        mod get_black_king_moves {            

            #[test]
            fn it_should_return_a_list_of_moves_from_startin_position() {
                assert!(true);       
            }
        }
    }
}