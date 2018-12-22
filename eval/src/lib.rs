extern crate board;
use std::rc::Rc;
use board::helpers::generate_board;
use board::helpers::index_to_square;
use board::Move;
use board::Board;
use board::square::Color;
use board::Piece;

pub fn get_all_legal_moves(board: Board) -> Vec<Move> {
    let test_board: Board = Board::new(board.squares.clone(), board.current_turn.color);
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

        #[test]
        fn it_finds_tweny_legal_moves_for_black_after_white_moves() {
            let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
            let mut board: Board = generate_board(board_string);
            board.make_move(Move::from_chess_move((String::from("e2"), String::from("e4"))));
            let legal_moves: Vec<Move> = get_all_legal_moves(board);
            assert_eq!(legal_moves.len(), 20);
        }

        #[test]
        fn it_finds_capture_moves_for_pawns() {
            let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
            let mut board: Board = generate_board(board_string);
            board.make_move(Move::from_chess_move((String::from("e2"), String::from("e4"))));
            board.make_move(Move::from_chess_move((String::from("d7"), String::from("d5"))));
            let legal_moves: Vec<Move> = get_all_legal_moves(board);
            assert_eq!(legal_moves.len(), 31);
        }
    }
}
