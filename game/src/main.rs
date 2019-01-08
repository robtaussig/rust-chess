extern crate board;
extern crate eval;
use std::env;

use board::helpers::generate_board;
use board::helpers::generate_board_with_current_turn;
use board::Move;
use board::square::Color;
use eval::get_all_legal_moves;
use eval::get_best_move;
use eval::get_best_move_rec;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
        let mut board: board::Board = generate_board(board_string);
        let (board, legal_moves) = get_all_legal_moves(board);
        let evaluation = get_best_move_rec(&board, 4, board.current_turn.color == Color::White, i32::min_value(), i32::max_value(), true);
        println!("{}", evaluation.1.unwrap());
    } else {
        let board_string = String::from(args[1].clone());
        let current_color: Color = match args[2].as_ref() {
            "-b" => Color::Black,
            _ => Color:: White,
        };
        let mut board: board::Board = generate_board_with_current_turn(board_string, current_color);
        let (board, legal_moves) = get_all_legal_moves(board);
        let evaluation = get_best_move_rec(&board, 5, board.current_turn.color == Color::White, i32::min_value(), i32::max_value(), true);
        println!("{}", evaluation.1.unwrap());
    }
}
