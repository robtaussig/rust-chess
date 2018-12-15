extern crate board;
extern crate eval;

use board::generate_board;

fn main() {
    let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
    let mut board: board::Board = generate_board(board_string);
}
