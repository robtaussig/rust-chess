extern crate board;
use std::rc::Rc;
use board::helpers::generate_board;
use board::Move;
use board::Board;
use board::square::Color;
use board::Piece;

pub fn get_all_legal_moves(board: Board) -> Vec<Move> {
    let current_color: Color = board.current_turn.color;
    let mut legal_moves: Vec<Move> = Vec::new();
    let test_board: Board = Board::new(board.squares.clone(), current_color);
    let legal_pieces: Vec<(usize, Piece)> = test_board.squares.into_iter()
        .enumerate()
        .filter(|&(_i, square)| match square.piece {
            Some(p) => {
                if p.color == current_color {
                    return true;
                }
                false
            },
            None => false,
        })
        .map(|(i, square)| (i, square.piece.unwrap()))
        .collect();

    for piece_with_index in legal_pieces.iter() {
        let test_board: Board = Board::new(board.squares.clone(), current_color);
        legal_moves.extend(piece_with_index.1.get_moves(piece_with_index.0 as usize, Rc::new(test_board)));
    }
    legal_moves
}

#[cfg(test)]
mod tests {
    use super::*;

    mod get_all_legal_moves {
        use super::*;

        #[test]
        fn it_finds_tweny_legal_moves_for_white_from_initial_position() {
            let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
            let board: Board = generate_board(board_string);
            let legal_moves: Vec<Move> = get_all_legal_moves(board);
            assert_eq!(legal_moves.len(), 20);
        }
    }
}
