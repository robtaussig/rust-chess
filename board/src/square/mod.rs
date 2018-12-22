use std::rc::Rc;
use std::fmt;
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

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self.piece {
            Some(p) => match p.color {
                Color::White => match p.piece_type {
                    PieceType::Pawn => "[P]",
                    PieceType::Knight => "[N]",
                    PieceType::Bishop => "[B]",
                    PieceType::Rook => "[R]",
                    PieceType::Queen => "[Q]",
                    PieceType::King => "[K]",
                },
                Color::Black => match p.piece_type {
                    PieceType::Pawn => "[p]",
                    PieceType::Knight => "[n]",
                    PieceType::Bishop => "[b]",
                    PieceType::Rook => "[r]",
                    PieceType::Queen => "[q]",
                    PieceType::King => "[k]",
                }
            },
            None => "[ ]",
        })
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
        match self.piece_type {
            PieceType::Pawn => valid_moves::get_pawn_moves(board_index as i8, board, self.color),
            PieceType::Knight => valid_moves::get_knight_moves(board_index as i8, board, self.color),
            PieceType::Bishop => valid_moves::get_bishop_moves(board_index as i8, board, self.color),
            PieceType::Rook => valid_moves::get_rook_moves(board_index as i8, board, self.color),
            PieceType::Queen => valid_moves::get_queen_moves(board_index as i8, board, self.color),
            PieceType::King => valid_moves::get_king_moves(board_index as i8, board, self.color),
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
#[derive(Copy, Clone)]
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

    const BISHOP_MOVE_DIRECTIONS: [i8; 4] = [9, 11, -9, -11];
    const KNIGHT_STEPPING_MOVES: [i8; 8] = [-12, -21, -19, -8, 12, 21, 19, 8];
    const KING_QUEEN_MOVE_DIRECTIONS: [i8; 8] = [-1, -11, -10, -9, 1, 11, 10, 9];
    const ROOK_MOVE_DIRECTIONS: [i8; 4] = [-1, 1, -10, 10];
    const WHITE_PAWN_MOVE_DIRECTIONS: [i8; 1] = [-10];
    const WHITE_PAWN_CAPTURE_DIRECTIONS: [i8; 2] = [-9, -11];
    const BLACK_PAWN_MOVE_DIRECTIONS: [i8; 1] = [10];
    const BLACK_PAWN_CAPTURE_DIRECTIONS: [i8; 2] = [9, 11];

    pub fn is_out_of_bounds(board_index: i8) -> bool {
        board_index < 11 || board_index > 88 || board_index % 10 == 0 || board_index % 10 == 9
    }

    pub fn get_pawn_moves(board_index: i8, board: Rc<Board>, color: Color) -> Vec<Move> {
        let move_directions: [i8; 1] = match color {
            Color::White => WHITE_PAWN_MOVE_DIRECTIONS,
            Color::Black => BLACK_PAWN_MOVE_DIRECTIONS,
        };
        let capture_directions: [i8; 2] = match color {
            Color::White => WHITE_PAWN_CAPTURE_DIRECTIONS,
            Color::Black => BLACK_PAWN_CAPTURE_DIRECTIONS,
        };
        let double_move_directions: [i8; 1] = match color {
            Color::White => match board_index {
                71 ... 78 => [-20],
                _ => [0],
            },
            Color::Black => match board_index {
                11 ... 18 => [20],
                _ => [0],
            },
        };

        let legal_moves: Vec<Move> = move_directions.into_iter()
            .map(|step| board_index + step)
            .filter(|to| {
                if is_out_of_bounds(*to) { return false; }
                match board.get_piece_at(*to as usize) {
                    Some(_) => false,
                    None => true
                }
            })
            .chain(
                capture_directions.into_iter()
                    .map(|step| board_index + step)
                    .filter(|to| {
                        if is_out_of_bounds(*to) { return false; }
                        match board.get_piece_at(*to as usize) {
                            Some(p) => p.color != color,
                            None => false
                        }
                    })
            )
            .chain(
                double_move_directions.into_iter()
                    .map(|step| board_index + step)
                    .filter(|to| {
                        if is_out_of_bounds(*to) { return false; }
                        match board.get_piece_at(*to as usize) {
                            Some(_) => false,
                            None => match board.get_piece_at((board_index as i8 + ((*to - board_index) / 2)) as usize) {
                                Some(_) => false,
                                None => true,
                            }
                        }
                    })
            )
            .map(|to| Move::new(board_index as usize, to as usize))
            .collect();

        legal_moves
    }

    pub fn get_knight_moves(board_index: i8, board: Rc<Board>, color: Color) -> Vec<Move> {
        let legal_moves: Vec<Move> = KNIGHT_STEPPING_MOVES.into_iter()
            .map(|step| board_index + step)
            .filter(|to| {
                if is_out_of_bounds(*to) { return false; }
                match board.get_piece_at(*to as usize) {
                    Some(p) => p.color != color,
                    None => true
                }
            })
            .map(|to| Move::new(board_index as usize, to as usize))
            .collect();

        legal_moves
    }

    pub fn get_bishop_moves(board_index: i8, board: Rc<Board>, color: Color) -> Vec<Move> {
        let mut legal_moves: Vec<Move> = Vec::new();

        for direction in BISHOP_MOVE_DIRECTIONS.into_iter() {
            let mut to: i8 = board_index;
            loop {
                to = to + direction;
                if is_out_of_bounds(to) { break; }
                match board.get_piece_at(to as usize) {
                    Some(p) => {
                        if p.color != color {
                            legal_moves.push(Move::new(board_index as usize, to as usize));
                        }
                        break;
                    },
                    None => {
                        legal_moves.push(Move::new(board_index as usize, to as usize));
                    },
                }
            }
        }

        legal_moves
    }

    pub fn get_rook_moves(board_index: i8, board: Rc<Board>, color: Color) -> Vec<Move> {
        let mut legal_moves: Vec<Move> = Vec::new();

        for direction in ROOK_MOVE_DIRECTIONS.into_iter() {
            let mut to: i8 = board_index;
            loop {
                to = to + direction;
                if is_out_of_bounds(to) { break; }
                match board.get_piece_at(to as usize) {
                    Some(p) => {
                        if p.color != color {
                            legal_moves.push(Move::new(board_index as usize, to as usize));
                        }
                        break;
                    },
                    None => {
                        legal_moves.push(Move::new(board_index as usize, to as usize));
                    },
                }
            }
        }

        legal_moves
    }

    pub fn get_queen_moves(board_index: i8, board: Rc<Board>, color: Color) -> Vec<Move> {
        let mut legal_moves: Vec<Move> = Vec::new();

        for direction in KING_QUEEN_MOVE_DIRECTIONS.into_iter() {
            let mut to: i8 = board_index;
            loop {
                to = to + direction;
                if is_out_of_bounds(to) { break; }
                match board.get_piece_at(to as usize) {
                    Some(p) => {
                        if p.color != color {
                            legal_moves.push(Move::new(board_index as usize, to as usize));
                        }
                        break;
                    },
                    None => {
                        legal_moves.push(Move::new(board_index as usize, to as usize));
                    },
                }
            }
        }

        legal_moves
    }

    pub fn get_king_moves(board_index: i8, board: Rc<Board>, color: Color) -> Vec<Move> {
        let legal_moves: Vec<Move> = KING_QUEEN_MOVE_DIRECTIONS.into_iter()
            .map(|step| board_index + step)
            .filter(|to| {
                if is_out_of_bounds(*to) { return false; }
                match board.get_piece_at(*to as usize) {
                    Some(p) => p.color != color,
                    None => true
                }
            })
            .map(|to| Move::new(board_index as usize, to as usize))
            .collect();

        legal_moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod valid_moves {
        use super::*;

        mod get_pawn_moves {            
            use super::*;

            #[test]
            fn it_should_return_a_list_of_moves_from_a_given_position() {
                let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
                let mut board: Board = helpers::generate_board(board_string);
                board.make_move(Move::from_chess_move((String::from("h2"), String::from("h4"))));
                board.make_move(Move::from_chess_move((String::from("e7"), String::from("e5"))));
                board.make_move(Move::from_chess_move((String::from("g2"), String::from("g4"))));
                board.make_move(Move::from_chess_move((String::from("e5"), String::from("e4"))));
                board.make_move(Move::from_chess_move((String::from("a2"), String::from("a4"))));
                board.make_move(Move::from_chess_move((String::from("e4"), String::from("e3"))));
                let pawn = board.get_piece_at(helpers::square_to_index(String::from("d2")) as usize).unwrap();
                let legal_moves: Vec<Move> = pawn.get_moves(helpers::square_to_index(String::from("d2")) as usize, Rc::new(board));
                assert_eq!(legal_moves.len(), 3);
            }
        }

        mod get_knight_moves {
            use super::*;
    
            #[test]
            fn it_should_return_a_list_of_moves_from_a_given_position() {
                let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
                let board: Board = helpers::generate_board(board_string);
                let knight = board.get_piece_at(helpers::square_to_index(String::from("g1")) as usize).unwrap();
                let legal_moves: Vec<Move> = knight.get_moves(helpers::square_to_index(String::from("g1")) as usize, Rc::new(board));
                assert_eq!(legal_moves.len(), 2);
            }
        }

        mod get_bishop_moves {            
            use super::*;

            #[test]
            fn it_should_return_a_list_of_moves_from_a_given_position() {
                let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
                let mut board: Board = helpers::generate_board(board_string);
                board.make_move(Move::from_chess_move((String::from("e2"), String::from("e4"))));
                board.make_move(Move::from_chess_move((String::from("e7"), String::from("e5"))));
                let bishop = board.get_piece_at(helpers::square_to_index(String::from("f1")) as usize).unwrap();
                let legal_moves: Vec<Move> = bishop.get_moves(helpers::square_to_index(String::from("f1")) as usize, Rc::new(board));
                assert_eq!(legal_moves.len(), 5);
            }
        }

        mod get_rook_moves {            
            use super::*;

            #[test]
            fn it_should_return_a_list_of_moves_from_a_given_position() {
                let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
                let mut board: Board = helpers::generate_board(board_string);
                board.make_move(Move::from_chess_move((String::from("h2"), String::from("h4"))));
                board.make_move(Move::from_chess_move((String::from("e7"), String::from("e5"))));
                board.make_move(Move::from_chess_move((String::from("g1"), String::from("f3"))));
                board.make_move(Move::from_chess_move((String::from("d7"), String::from("d5"))));
                let rook = board.get_piece_at(88 as usize).unwrap();
                let legal_moves: Vec<Move> = rook.get_moves(88 as usize, Rc::new(board));
                assert_eq!(legal_moves.len(), 3);
            }
        }

        mod get_queen_moves {            
            use super::*;

            #[test]
            fn it_should_return_a_list_of_moves_from_a_given_position() {
                let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
                let mut board: Board = helpers::generate_board(board_string);
                board.make_move(Move::from_chess_move((String::from("e2"), String::from("e4"))));
                board.make_move(Move::from_chess_move((String::from("e7"), String::from("e5"))));
                board.make_move(Move::from_chess_move((String::from("d1"), String::from("g4"))));
                board.make_move(Move::from_chess_move((String::from("h7"), String::from("h4"))));
                let queen = board.get_piece_at(57 as usize).unwrap();
                let legal_moves: Vec<Move> = queen.get_moves(57 as usize, Rc::new(board));
                assert_eq!(legal_moves.len(), 14);
            }
        }

        mod get_king_moves {            
            use super::*;
            #[test]
            fn it_should_return_a_list_of_moves_from_a_given_position() {
                let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
                let mut board: Board = helpers::generate_board(board_string);
                board.make_move(Move::from_chess_move((String::from("e2"), String::from("e4"))));
                board.make_move(Move::from_chess_move((String::from("e7"), String::from("e5"))));
                board.make_move(Move::from_chess_move((String::from("e1"), String::from("e2"))));
                board.make_move(Move::from_chess_move((String::from("h7"), String::from("h4"))));
                let king = board.get_piece_at(75 as usize).unwrap();
                let legal_moves: Vec<Move> = king.get_moves(75 as usize, Rc::new(board));
                assert_eq!(legal_moves.len(), 4);     
            }
        }
    }
}