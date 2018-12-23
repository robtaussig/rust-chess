extern crate board;
use std::rc::Rc;
use board::Move;
use board::Board;
use board::helpers::generate_board;
use board::square::Color;
use board::Piece;
use board::PieceType;

pub fn get_all_legal_moves(board: Board) -> (Board, Vec<Move>) {
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
            let test_board: Board = Board::new(board.squares.clone(), board.current_turn.color);
            square.piece.unwrap().get_moves(i, Rc::new(test_board))
        })
        .collect();

    (board, legal_moves)
}

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

pub fn get_value_of_white_piece(piece: Piece, board_index: usize) -> u32 {
    match piece.piece_type {
        PieceType::Pawn =>  WHITE_PAWN_POSITIONAL_VALUE[board_index] + PAWN_VALUE,
        PieceType::Knight =>  WHITE_KNIGHT_POSITIONAL_VALUE[board_index] + KNIGHT_VALUE,
        PieceType::Bishop =>  WHITE_BISHOP_POSITIONAL_VALUE[board_index] + BISHOP_VALUE,
        PieceType::Rook =>  WHITE_ROOK_POSITIONAL_VALUE[board_index] + ROOK_VALUE,
        PieceType::Queen =>  WHITE_QUEEN_POSITIONAL_VALUE[board_index] + QUEEN_VALUE,
        PieceType::King => WHITE_KING_POSITIONAL_VALUE[board_index] + KING_VALUE,
    }
}

pub fn get_value_of_black_piece(piece: Piece, board_index: usize) -> u32 {
    match piece.piece_type {
        PieceType::Pawn =>  BLACK_PAWN_POSITIONAL_VALUE[board_index] + PAWN_VALUE,
        PieceType::Knight =>  BLACK_KNIGHT_POSITIONAL_VALUE[board_index] + KNIGHT_VALUE,
        PieceType::Bishop =>  BLACK_BISHOP_POSITIONAL_VALUE[board_index] + BISHOP_VALUE,
        PieceType::Rook =>  BLACK_ROOK_POSITIONAL_VALUE[board_index] + ROOK_VALUE,
        PieceType::Queen =>  BLACK_QUEEN_POSITIONAL_VALUE[board_index] + QUEEN_VALUE,
        PieceType::King => BLACK_KING_POSITIONAL_VALUE[board_index] + KING_VALUE,
    }
}

pub fn get_white_evaluation(board: &Board) -> u32 {
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

pub fn get_black_evaluation(board: &Board) -> u32 {
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

pub fn get_current_evaluation(board: Board) -> (u32, u32) {
    (get_white_evaluation(&board), get_black_evaluation(&board))
}

pub fn get_best_move(board: &Board, depth: u32) -> Move {
    let board = board.clone();
    let current_color: Color = board.current_turn.color;
    let (board, legal_moves) = get_all_legal_moves(board);
    let best_move: &Move = legal_moves.iter()
        .map(|legal_move| {
            let next_board = board.test_move(Move { from: legal_move.from, to: legal_move.to });
            let evaluations = get_current_evaluation(next_board);
            match current_color {
                Color::White => (legal_move, evaluations.0 as i32 - evaluations.1 as i32),
                Color::Black => (legal_move, evaluations.1 as i32 - evaluations.0 as i32),
            }                
        })
        .max_by(|x, y| x.1.cmp(&y.1))
        .map(|(legal_move, _)| legal_move)
        .unwrap();
    
    Move::new(best_move.from, best_move.to)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod eval {
        use super::*;

        #[test]
        fn it_evaluates_a_board_position() {
            let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
            let board: Board = generate_board(board_string);
            let (white_value, black_value): (u32, u32) = get_current_evaluation(board);

            assert_eq!(white_value, black_value);
        }

        mod get_best_move {
            use super::*;

            #[test]
            fn it_gives_best_move_with_one_depth() {
                let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
                let board: Board = generate_board(board_string);
                let best_move: Move = get_best_move(&board, 1);

                let expected_best_move = Move::from_chess_move((String::from("E2"), String::from("E4")));
                assert_eq!((best_move.from, best_move.to), (expected_best_move.from, expected_best_move.to));
            }
        }

        mod get_white_evaluation {
            use super::*;

            #[test]
            fn it_gets_the_value_for_whites_starting_position() {
                let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
                let board: Board = generate_board(board_string);
                let white_value: u32 = get_white_evaluation(&board);
                assert_eq!(white_value, 14420);
            }

            #[test]
            fn it_gets_the_value_for_blacks_starting_position() {
                let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
                let board: Board = generate_board(board_string);
                let black_value: u32 = get_black_evaluation(&board);
                assert_eq!(black_value, 14420);
            }
        }
    }

    mod get_all_legal_moves {
        use super::*;

        #[test]
        fn it_finds_tweny_legal_moves_for_white_from_initial_position() {
            let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
            let board: Board = generate_board(board_string);
            let (board, legal_moves) = get_all_legal_moves(board);
            assert_eq!(legal_moves.len(), 20);
        }

        #[test]
        fn it_finds_tweny_legal_moves_for_black_after_white_moves() {
            let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
            let mut board: Board = generate_board(board_string);
            board.make_move(Move::from_chess_move((String::from("e2"), String::from("e4"))));
            let (board, legal_moves) = get_all_legal_moves(board);
            assert_eq!(legal_moves.len(), 20);
        }

        #[test]
        fn it_finds_capture_moves_for_pawns() {
            let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
            let mut board: Board = generate_board(board_string);
            board.make_move(Move::from_chess_move((String::from("e2"), String::from("e4"))));
            board.make_move(Move::from_chess_move((String::from("d7"), String::from("d5"))));
            let (board, legal_moves) = get_all_legal_moves(board);
            assert_eq!(legal_moves.len(), 31);
        }

        #[test]
        fn it_doesnt_leave_player_in_check() {
            let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
            let mut board: Board = generate_board(board_string);
            board.make_move(Move::from_chess_move((String::from("e2"), String::from("e4"))));
            board.make_move(Move::from_chess_move((String::from("d7"), String::from("d5"))));
            board.make_move(Move::from_chess_move((String::from("d1"), String::from("h5"))));
            let (board, legal_moves) = get_all_legal_moves(board.clone());
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
