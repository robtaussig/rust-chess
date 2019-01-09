extern crate board;
extern crate eval;
use std::env;
use board::Move;
use board::Board;
use board::square::Color;
use eval::get_all_legal_moves;
use eval::get_best_move;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
        let mut board = Board::new(board_string, Color::White);
        let legal_moves = get_all_legal_moves(&board);
        let evaluation = get_best_move(&board, 4);
        println!("{}", evaluation.unwrap());
    } else {
        let board_string = String::from(args[1].clone());
        let current_color: Color = match args[2].as_ref() {
            "-b" => Color::Black,
            _ => Color:: White,
        };
        let mut board: Board = Board::new(board_string, current_color);
        let legal_moves = get_all_legal_moves(&board);
        let evaluation = get_best_move(&board, 4);
        println!("{}", evaluation.unwrap());
    }
}
