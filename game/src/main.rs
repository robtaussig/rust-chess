extern crate board;
extern crate eval;

use board::helpers::generate_board;
use board::Move;

fn main() {
    let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
    let mut board: board::Board = generate_board(board_string);
    board.make_move(Move::new(75, 55));
    println!("{}", board);
}
