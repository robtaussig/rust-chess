extern crate board;
use std::collections::HashMap;
#[macro_use] extern crate itertools;
use std::cmp;
use itertools::Itertools;
use std::rc::Rc;
use board::Move;
use board::Board;
use board::square::Color;
use board::Piece;
use board::PieceType;

const BLACK_PAWN_POSITIONAL_VALUE: [u32; 100] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 20, 20, 20, 20, 20, 50, 50, 50, 0,
    0, 30, 30, 40, 40, 40, 20, 40, 40, 0,
    0, 10, 10, 50, 50, 50, 10, 30, 30, 0,
    0, 20, 20, 40, 40, 40, 20, 20, 20, 0,
    0, 20, 20, 30, 30, 30, 20, 20, 20, 0,
    0, 20, 20, 20, 20, 20, 20, 20, 20, 0,
    0, 100, 100, 100, 100, 100, 100, 100, 100, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

const WHITE_PAWN_POSITIONAL_VALUE: [u32; 100] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 100, 100, 100, 100, 100, 100, 100, 100, 0,
    0, 20, 20, 20, 20, 20, 20, 20, 20, 0,
    0, 20, 20, 30, 30, 30, 20, 20, 20, 0,
    0, 20, 20, 40, 40, 40, 20, 20, 20, 0,
    0, 10, 10, 50, 50, 50, 10, 30, 30, 0,
    0, 30, 30, 40, 40, 40, 20, 40, 40, 0,
    0, 20, 20, 20, 20, 20, 50, 50, 50, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

const BLACK_KNIGHT_POSITIONAL_VALUE: [u32; 100] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 30, 10, 10, 10, 10, 30, 0, 0,
    0, 10, 20, 20, 30, 30, 20, 20, 10, 0,
    0, 10, 20, 40, 40, 40, 40, 20, 10, 0,
    0, 20, 30, 50, 50, 50, 50, 30, 20, 0,
    0, 20, 30, 50, 50, 50, 50, 30, 20, 0,
    0, 10, 20, 40, 40, 40, 40, 20, 10, 0,
    0, 10, 20, 20, 30, 30, 20, 20, 10, 0,
    0, 0, 10, 10, 10, 10, 10, 10, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

const WHITE_KNIGHT_POSITIONAL_VALUE: [u32; 100] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 10, 10, 10, 10, 10, 10, 0, 0,
    0, 10, 20, 20, 30, 30, 20, 20, 10, 0,
    0, 10, 20, 40, 40, 40, 40, 20, 10, 0,
    0, 20, 30, 50, 50, 50, 50, 30, 20, 0,
    0, 20, 30, 50, 50, 50, 50, 30, 20, 0,
    0, 10, 20, 40, 40, 40, 40, 20, 10, 0,
    0, 10, 20, 20, 30, 30, 20, 20, 10, 0,
    0, 0, 30, 10, 10, 10, 10, 30, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

const BLACK_ROOK_POSITIONAL_VALUE: [u32; 100] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 50, 20, 20, 50, 50, 50, 0, 50, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 30, 30, 30, 30, 30, 30, 30, 30, 0,
    0, 30, 30, 30, 30, 30, 30, 30, 30, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

const WHITE_ROOK_POSITIONAL_VALUE: [u32; 100] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 30, 30, 30, 30, 30, 30, 30, 30, 0,
    0, 30, 30, 30, 30, 30, 30, 30, 30, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 50, 20, 20, 50, 50, 50, 0, 50, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

const BLACK_BISHOP_POSITIONAL_VALUE: [u32; 100] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 30, 0, 0, 30, 0, 0, 0,
    0, 0, 40, 0, 30, 30, 0, 40, 0, 0,
    0, 0, 0, 20, 20, 20, 20, 0, 0, 0,
    0, 0, 20, 40, 30, 30, 40, 20, 0, 0,
    0, 0, 40, 30, 20, 20, 30, 40, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

const WHITE_BISHOP_POSITIONAL_VALUE: [u32; 100] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 40, 30, 20, 20, 30, 40, 0, 0,
    0, 0, 20, 40, 30, 30, 40, 20, 0, 0,
    0, 0, 0, 20, 20, 20, 20, 0, 0, 0,
    0, 0, 40, 0, 30, 30, 0, 40, 0, 0,
    0, 0, 0, 30, 0, 0, 30, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

const BLACK_QUEEN_POSITIONAL_VALUE: [u32; 100] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

const WHITE_QUEEN_POSITIONAL_VALUE: [u32; 100] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

//100's encourage castling if possible. Otherwise stay in place.
const BLACK_KING_POSITIONAL_VALUE: [u32; 100] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 100, 100, 0, 50, 0, 100, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

const WHITE_KING_POSITIONAL_VALUE: [u32; 100] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 100, 100, 0, 50, 0, 100, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

const PAWN_VALUE: u32 = 100;
const BISHOP_VALUE: u32 = 300;
const KNIGHT_VALUE: u32 = 300;
const ROOK_VALUE: u32 = 500;
const QUEEN_VALUE: u32 = 900;
const KING_VALUE: u32 = 10000;

pub struct Eval {
    cache: HashMap<String, (u32, i32)>
}

impl Eval {
    pub fn new(board: &Board, depth: u32) -> Option<Move> {
        let mut evaluation = Eval { cache: HashMap::new() };
        evaluation.get_best_move(board, depth)
    }

    fn get_best_move(&mut self, board: &Board, depth: u32) -> Option<Move> {
        let mut evaluation_cache: HashMap<String, (u32, i32)> = HashMap::new();
        let min_max_evaluation = self.min_max_evaluation(board, depth, true, i32::min_value(), i32::max_value(), true);
        min_max_evaluation.1
    }

    fn min_max_evaluation(&mut self, board: &Board, depth: u32, is_maximizer: bool, alpha: i32, beta: i32, root: bool) -> (i32, Option<Move>) {
        if let Some(cached_result) = self.cache.get(&board.board_string_with_turn_bit) {
            let (cached_depth, best_move_value): (u32, i32) = *cached_result;
            return (best_move_value.clone(), None);
        }
        
        if depth == 0 {
            let evaluations = get_snapshot_evaluation(&board);
            let value = match board.current_turn.color {
                Color::White => evaluations.0 as i32 - evaluations.1 as i32,
                Color::Black => evaluations.1 as i32 - evaluations.0 as i32,
            };

            if is_maximizer == true {
                return (value, None);
            } else {
                return (-value, None);
            }
        }

        let legal_moves = get_legal_moves_sorted_by_strength(&board);

        let (best_move_value, best_move) = self.get_best_move_with_value(&board, depth, legal_moves, is_maximizer, alpha, beta);
        self.cache.insert(board.board_string_with_turn_bit.clone(), (depth, best_move_value));
        (best_move_value, best_move)
    }

    fn get_best_move_with_value(&mut self, board: &Board, depth: u32, legal_moves: Vec<Move>, is_maximizer:bool, mut alpha: i32, mut beta: i32) -> (i32, Option<Move>) {
        let mut best_move: Option<Move> = None;
        let mut best_move_value = match is_maximizer {
            true => i32::min_value(),
            false => i32::max_value()
        };

        for legal_move in legal_moves.into_iter() {
            let next_board = board.test_move(Move { from: legal_move.from, to: legal_move.to });
            let value: i32 = self.min_max_evaluation(&next_board, depth - 1, !is_maximizer, alpha, beta, false).0;

            match is_maximizer {
                true => {
                    if value > best_move_value {
                        best_move_value = value;
                        best_move = Some(legal_move);
                    }
                    alpha = cmp::max(alpha, value);
                },
                false => {
                    if value < best_move_value {
                        best_move_value = value;
                        best_move = Some(legal_move);
                    }
                    beta = cmp::min(beta, value);
                },
            };

            if beta <= alpha {
                break;
            }
        }
        (best_move_value, best_move)
    }
}

pub fn get_all_legal_moves(board: &Board) -> Vec<Move> {
    let test_board: Board = board.clone();
    let legal_moves: Vec<Move> = test_board.squares.into_iter()
        .enumerate()
        .filter(|&(_i, square)| match square.piece {
            Some(p) => {
                if p.color == board.current_turn.color {
                    return true;
                }
                false
            },
            None => false,
        })
        .flat_map(|(i, square)| {
            let test_board = board.clone();
            square.piece.unwrap().get_moves(i, Rc::new(test_board))
        })
        .collect();

    legal_moves
}

fn get_legal_moves_sorted_by_strength(board: &Board) -> Vec<Move> {
    let legal_moves = get_all_legal_moves(board);
    let legal_moves_sorted: Vec<Move> = legal_moves.into_iter()
        .map(|legal_move| {
            let next_board = board.test_move(Move { from: legal_move.from, to: legal_move.to });
            let evaluations = get_snapshot_evaluation(&next_board);
            match next_board.current_turn.color {
                Color::White => (legal_move, evaluations.0 as i32 - evaluations.1 as i32),
                Color::Black => (legal_move, evaluations.1 as i32 - evaluations.0 as i32),
            }
        })
        .sorted_by(|x, y| x.1.cmp(&y.1))
        .map(|(legal_move, _)| legal_move)
        .collect();
    legal_moves_sorted
}

fn get_value_of_white_piece(piece: Piece, board_index: usize) -> u32 {
    match piece.piece_type {
        PieceType::Pawn =>  WHITE_PAWN_POSITIONAL_VALUE[board_index] + PAWN_VALUE,
        PieceType::Knight =>  WHITE_KNIGHT_POSITIONAL_VALUE[board_index] + KNIGHT_VALUE,
        PieceType::Bishop =>  WHITE_BISHOP_POSITIONAL_VALUE[board_index] + BISHOP_VALUE,
        PieceType::Rook =>  WHITE_ROOK_POSITIONAL_VALUE[board_index] + ROOK_VALUE,
        PieceType::Queen =>  WHITE_QUEEN_POSITIONAL_VALUE[board_index] + QUEEN_VALUE,
        PieceType::King => WHITE_KING_POSITIONAL_VALUE[board_index] + KING_VALUE,
    }
}

fn get_value_of_black_piece(piece: Piece, board_index: usize) -> u32 {
    match piece.piece_type {
        PieceType::Pawn =>  BLACK_PAWN_POSITIONAL_VALUE[board_index] + PAWN_VALUE,
        PieceType::Knight =>  BLACK_KNIGHT_POSITIONAL_VALUE[board_index] + KNIGHT_VALUE,
        PieceType::Bishop =>  BLACK_BISHOP_POSITIONAL_VALUE[board_index] + BISHOP_VALUE,
        PieceType::Rook =>  BLACK_ROOK_POSITIONAL_VALUE[board_index] + ROOK_VALUE,
        PieceType::Queen =>  BLACK_QUEEN_POSITIONAL_VALUE[board_index] + QUEEN_VALUE,
        PieceType::King => BLACK_KING_POSITIONAL_VALUE[board_index] + KING_VALUE,
    }
}

fn get_white_evaluation(board: &Board) -> u32 {
    let mut value: u32 = 0;
    let board = board.clone();
    board.squares.into_iter()
        .enumerate()
        .for_each(|(i, square)| {
            value = value + match square.piece {
                Some(p) => match p.color {
                    Color::White => get_value_of_white_piece(p, i),
                    _ => 0
                },
                None => 0,
            }
        });

    value
}

fn get_black_evaluation(board: &Board) -> u32 {
    let mut value: u32 = 0;
    let board = board.clone();
    board.squares.into_iter()
        .enumerate()
        .for_each(|(i, square)| {
            value = value + match square.piece {
                Some(p) => match p.color {
                    Color::Black => get_value_of_black_piece(p, i),
                    _ => 0
                },
                None => 0,
            }
        });

    value
}

fn get_snapshot_evaluation(board: &Board) -> (u32, u32) {
    (get_white_evaluation(board), get_black_evaluation(board))
}

#[cfg(test)]
mod tests {
    use super::*;

    mod eval {
        use super::*;

        #[test]
        fn it_evaluates_a_board_position() {
            let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
            let board: Board = Board::new(board_string, Color::White);
            let (white_value, black_value): (u32, u32) = get_snapshot_evaluation(&board);

            assert_eq!(white_value, black_value);
        }

        mod get_best_move {
            use super::*;

            #[test]
            fn it_gives_best_move_with_one_depth() {
                let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
                let board: Board = Board::new(board_string, Color::White);
                let best_move: Move = Eval::new(&board, 1).unwrap();

                let expected_best_move = Move::from_chess_move((String::from("c2"), String::from("c4")));
                assert_eq!((best_move.from, best_move.to), (expected_best_move.from, expected_best_move.to));
            }

            #[test]
            fn it_blunders_pieces_with_one_depth() {
                let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
                let mut board: Board = Board::new(board_string, Color::White);
                board.make_move(Move::from_chess_move((String::from("d2"), String::from("d4"))));
                board.make_move(Move::from_chess_move((String::from("e7"), String::from("e5"))));
                board.make_move(Move::from_chess_move((String::from("h2"), String::from("h4"))));
                board.make_move(Move::from_chess_move((String::from("h7"), String::from("h5"))));
                board.make_move(Move::from_chess_move((String::from("c1"), String::from("g5"))));
                let best_move: Move = Eval::new(&board, 1).unwrap();
                let expected_best_move = Move::from_chess_move((String::from("d8"), String::from("g5")));
                assert_eq!((best_move.from, best_move.to), (expected_best_move.from, expected_best_move.to));
            }

            #[test]
            fn it_explores_move_tree_given_depth() {
                let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
                let mut board: Board = Board::new(board_string, Color::White);
                board.make_move(Move::from_chess_move((String::from("d2"), String::from("d4"))));
                board.make_move(Move::from_chess_move((String::from("e7"), String::from("e5"))));
                board.make_move(Move::from_chess_move((String::from("h2"), String::from("h4"))));
                board.make_move(Move::from_chess_move((String::from("h7"), String::from("h5"))));
                board.make_move(Move::from_chess_move((String::from("c1"), String::from("g5"))));
                let best_move: Move = Eval::new(&board, 4).unwrap();
                let expected_best_move = Move::from_chess_move((String::from("f7"), String::from("f6")));
                assert_eq!((best_move.from, best_move.to), (expected_best_move.from, expected_best_move.to));
            }
        }

        mod get_white_evaluation {
            use super::*;

            #[test]
            fn it_gets_the_value_for_whites_starting_position() {
                let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
                let board: Board = Board::new(board_string, Color::White);
                let white_value: u32 = get_white_evaluation(&board);
                assert_eq!(white_value, 14420);
            }

            #[test]
            fn it_gets_the_value_for_blacks_starting_position() {
                let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
                let board: Board = Board::new(board_string, Color::White);
                let black_value: u32 = get_black_evaluation(&board);
                assert_eq!(black_value, 14420);
            }
        }
    }

    mod get_all_legal_moves {
        use super::*;

        #[test]
        fn it_finds_twenty_legal_moves_for_white_from_initial_position() {
            let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
            let board: Board = Board::new(board_string, Color::White);
            let legal_moves = get_all_legal_moves(&board);
            assert_eq!(legal_moves.len(), 20);
        }

        #[test]
        fn it_finds_twenty_legal_moves_for_black_after_white_moves() {
            let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
            let mut board: Board = Board::new(board_string, Color::White);
            board.make_move(Move::from_chess_move((String::from("e2"), String::from("e4"))));
            let legal_moves = get_all_legal_moves(&board);
            assert_eq!(legal_moves.len(), 20);
        }

        #[test]
        fn it_finds_capture_moves_for_pawns() {
            let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
            let mut board: Board = Board::new(board_string, Color::White);
            board.make_move(Move::from_chess_move((String::from("e2"), String::from("e4"))));
            board.make_move(Move::from_chess_move((String::from("d7"), String::from("d5"))));
            let legal_moves = get_all_legal_moves(&board);
            assert_eq!(legal_moves.len(), 31);
        }

        #[test]
        fn it_doesnt_leave_player_in_check() {
            let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
            let mut board: Board = Board::new(board_string, Color::White);
            board.make_move(Move::from_chess_move((String::from("e2"), String::from("e4"))));
            board.make_move(Move::from_chess_move((String::from("d7"), String::from("d5"))));
            board.make_move(Move::from_chess_move((String::from("d1"), String::from("h5"))));
            let legal_moves = get_all_legal_moves(&board);
            let legal_moves_include_f7_pawn = match legal_moves.iter().find(|legal_move| match legal_move.from {
                26 => true,
                _ => false,
            }) {
                Some(_) => true,
                None => false,
            };
            assert_eq!(legal_moves_include_f7_pawn, false);
        }
    }
}
