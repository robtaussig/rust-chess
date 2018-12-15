extern crate arrayvec;
use arrayvec::ArrayVec;

pub struct Square {
    pub piece: Option<Piece>,
    pub is_edge: bool,
}

pub enum Color {
    White, Black
}

pub enum Piece {
    Pawn(Color), Knight(Color), Bishop(Color), Rook(Color), Queen(Color), King(Color)
}

pub struct Move {
    from: u8,
    to: u8,
}

pub struct Board {
    pub squares: ArrayVec<[Square; 100]>
}

impl Board {
    pub fn get_legal_moves() -> Vec<Move> {
        Vec::new()
    }
}

fn generate_square_from_string(square: char) -> Square {
    match square {
        'p' => Square { piece: Some(Piece::Pawn(Color::Black)), is_edge: false },
        'P' => Square { piece: Some(Piece::Pawn(Color::White)), is_edge: false },
        'b' => Square { piece: Some(Piece::Bishop(Color::Black)), is_edge: false },
        'B' => Square { piece: Some(Piece::Bishop(Color::White)), is_edge: false },
        'n' => Square { piece: Some(Piece::Knight(Color::Black)), is_edge: false },
        'N' => Square { piece: Some(Piece::Knight(Color::White)), is_edge: false },
        'r' => Square { piece: Some(Piece::Rook(Color::Black)), is_edge: false },
        'R' => Square { piece: Some(Piece::Rook(Color::White)), is_edge: false },
        'q' => Square { piece: Some(Piece::Queen(Color::Black)), is_edge: false },
        'Q' => Square { piece: Some(Piece::Queen(Color::White)), is_edge: false },
        'k' => Square { piece: Some(Piece::King(Color::Black)), is_edge: false },
        'K' => Square { piece: Some(Piece::King(Color::White)), is_edge: false },
        '0' => Square { piece: None, is_edge: true },
        '-' => Square { piece: None, is_edge: false },
        _ => panic!("Received piece char other than accepted values")
    }
}

pub fn generate_board(mut board_string: String) -> Board {
    if (board_string.len() != 100) {
        panic!("The board must be of length 100 to be accepted. Received board was of length {}", board_string.len());
    }
    let mut squares = ArrayVec::<[Square; 100]>::new();

    for square in board_string.chars() {
        squares.push(generate_square_from_string(square));
    }

    Board { squares }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod generate_board {
        use super::*;
        
        #[test]
        #[should_panic]
        fn it_panics_when_board_is_missing_square() {
            let board_string_missing_one = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR0000000000");
            let mut board: Board = generate_board(board_string_missing_one);
        }

        #[test]
        #[should_panic]
        fn it_panics_when_board_has_two_many_squares() {
            let board_string_missing_one = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR000000000000");
            let mut board: Board = generate_board(board_string_missing_one);
        }

        #[test]
        #[should_panic]
        fn it_panics_when_board_has_unknown_piece() {
            let board_string_missing_one = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNzQKBNR0000000000");
            let mut board: Board = generate_board(board_string_missing_one);
        }
    }
}