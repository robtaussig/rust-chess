extern crate arrayvec;
use arrayvec::ArrayVec;
use std::fmt;
pub mod square;
pub use self::square::{ Square, Piece, Color, Move, Turn, PieceType };

#[derive(Clone)]
pub struct Board {
    pub squares: ArrayVec<[Square; 100]>,
    pub current_turn: Turn,
    pub board_string_with_turn_bit: String,
}

impl Board {

    pub fn new(board_string: String, current_color: Color) -> Board {
        if board_string.len() != 100 {
            panic!("The board must be of length 100 to be accepted. Received board was of length {}", board_string.len());
        }

        let mut squares = ArrayVec::<[Square; 100]>::new();

        for square in board_string.chars() {
            squares.push(helpers::generate_square_from_string(square));
        }

        let mut board_string_with_turn_bit = board_string.clone();
        board_string_with_turn_bit.push(match current_color {
            Color::White => '1',
            Color::Black => '0',
        });
        Board { squares, current_turn: Turn { color: current_color }, board_string_with_turn_bit }
    }

    pub fn get_piece_at(&self, index: usize) -> Option<Piece> {
        self.squares[index].piece
    }

    fn set_square(&mut self, index: usize, piece: Option<Piece>) {
        match piece {
            Some(p) => self.squares[index] = Square::new(p),
            None => self.squares[index] = Square { piece: None, is_edge: false },
        }
    }

    fn get_piece_char_at(&self, index: usize) -> char {
        let square = &self.squares[index];
        match square.piece {
            Some(p) => match p.color {
                Color::White => match p.piece_type {
                    PieceType::Pawn => 'P',
                    PieceType::Knight => 'N',
                    PieceType::Bishop => 'B',
                    PieceType::Rook => 'R',
                    PieceType::Queen => 'Q',
                    PieceType::King => 'K',
                },
                Color::Black => match p.piece_type {
                    PieceType::Pawn => 'p',
                    PieceType::Knight => 'n',
                    PieceType::Bishop => 'b',
                    PieceType::Rook => 'r',
                    PieceType::Queen => 'q',
                    PieceType::King => 'k',
                }
            },
            None => match square.is_edge {
                true => '0',
                false => '-'
            }
        }
    }

    pub fn make_move(&mut self, chess_move: Move) {
        let from_piece_char = self.get_piece_char_at(chess_move.from);
        match self.get_piece_at(chess_move.from) {
            None => panic!("There is no piece on the square form which the move is being made: {:?}", self.squares[chess_move.from]),
            Some(p) => {
                self.current_turn.toggle();
                self.set_square(chess_move.from, None);
                self.set_square(chess_move.to, Some(p));
            }
        }
        let next_board_string: String = self.board_string_with_turn_bit.chars().enumerate()
            .map(|(i, c)| {
                if i == 100 {
                    match self.current_turn.color {
                        Color::White => '1',
                        Color::Black => '0',
                    }
                } else if i == chess_move.from {
                    '-'
                } else if i == chess_move.to {
                    from_piece_char
                } else {
                    c
                }
            })
            .collect();

        self.board_string_with_turn_bit = next_board_string;
    }

    pub fn clone(&self) -> Board {
        Board { squares: self.squares.clone(), current_turn: self.current_turn, board_string_with_turn_bit: self.board_string_with_turn_bit.clone() }
    }

    pub fn test_move(&self, chess_move: Move) -> Board {
        let mut test_board = self.clone();
        test_board.make_move(chess_move);
        test_board
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"
            {}{}{}{}{}{}{}{}
            {}{}{}{}{}{}{}{}
            {}{}{}{}{}{}{}{}
            {}{}{}{}{}{}{}{}
            {}{}{}{}{}{}{}{}
            {}{}{}{}{}{}{}{}
            {}{}{}{}{}{}{}{}
            {}{}{}{}{}{}{}{}
        ",self.squares[11 as usize], self.squares[12 as usize], self.squares[13 as usize], self.squares[14 as usize], self.squares[15 as usize], self.squares[16 as usize], self.squares[17 as usize], self.squares[18 as usize],
        self.squares[21 as usize], self.squares[22 as usize], self.squares[23 as usize], self.squares[24 as usize], self.squares[25 as usize], self.squares[26 as usize], self.squares[27 as usize], self.squares[28 as usize],
        self.squares[31 as usize], self.squares[32 as usize], self.squares[33 as usize], self.squares[34 as usize], self.squares[35 as usize], self.squares[36 as usize], self.squares[37 as usize], self.squares[38 as usize],
        self.squares[41 as usize], self.squares[42 as usize], self.squares[43 as usize], self.squares[44 as usize], self.squares[45 as usize], self.squares[46 as usize], self.squares[47 as usize], self.squares[48 as usize],
        self.squares[51 as usize], self.squares[52 as usize], self.squares[53 as usize], self.squares[54 as usize], self.squares[55 as usize], self.squares[56 as usize], self.squares[57 as usize], self.squares[58 as usize],
        self.squares[61 as usize], self.squares[62 as usize], self.squares[63 as usize], self.squares[64 as usize], self.squares[65 as usize], self.squares[66 as usize], self.squares[67 as usize], self.squares[68 as usize],
        self.squares[71 as usize], self.squares[72 as usize], self.squares[73 as usize], self.squares[74 as usize], self.squares[75 as usize], self.squares[76 as usize], self.squares[77 as usize], self.squares[78 as usize],
        self.squares[81 as usize], self.squares[82 as usize], self.squares[83 as usize], self.squares[84 as usize], self.squares[85 as usize], self.squares[86 as usize], self.squares[87 as usize], self.squares[88 as usize])
    }
}

pub mod helpers {
    use super::*;

    pub fn square_to_index(square: String) -> usize {
        match square.to_lowercase().as_ref() {
            "a8" => 11, "b8" => 12, "c8" => 13, "d8" => 14, "e8" => 15, "f8" => 16, "g8" => 17, "h8" => 18,
            "a7" => 21, "b7" => 22, "c7" => 23, "d7" => 24, "e7" => 25, "f7" => 26, "g7" => 27, "h7" => 28,
            "a6" => 31, "b6" => 32, "c6" => 33, "d6" => 34, "e6" => 35, "f6" => 36, "g6" => 37, "h6" => 38,
            "a5" => 41, "b5" => 42, "c5" => 43, "d5" => 44, "e5" => 45, "f5" => 46, "g5" => 47, "h5" => 48,
            "a4" => 51, "b4" => 52, "c4" => 53, "d4" => 54, "e4" => 55, "f4" => 56, "g4" => 57, "h4" => 58,
            "a3" => 61, "b3" => 62, "c3" => 63, "d3" => 64, "e3" => 65, "f3" => 66, "g3" => 67, "h3" => 68,
            "a2" => 71, "b2" => 72, "c2" => 73, "d2" => 74, "e2" => 75, "f2" => 76, "g2" => 77, "h2" => 78,
            "a1" => 81, "b1" => 82, "c1" => 83, "d1" => 84, "e1" => 85, "f1" => 86, "g1" => 87, "h1" => 88,
            _ => panic!("an illegal square was passed in: {}", square),
        }
    }

    pub fn index_to_square(index: usize) -> String {
        let square = match index {
            11 => "a8", 12 => "b8", 13 => "c8", 14 => "d8", 15 => "e8", 16 => "f8", 17 => "g8", 18 => "h8",
            21 => "a7", 22 => "b7", 23 => "c7", 24 => "d7", 25 => "e7", 26 => "f7", 27 => "g7", 28 => "h7",
            31 => "a6", 32 => "b6", 33 => "c6", 34 => "d6", 35 => "e6", 36 => "f6", 37 => "g6", 38 => "h6",
            41 => "a5", 42 => "b5", 43 => "c5", 44 => "d5", 45 => "e5", 46 => "f5", 47 => "g5", 48 => "h5",
            51 => "a4", 52 => "b4", 53 => "c4", 54 => "d4", 55 => "e4", 56 => "f4", 57 => "g4", 58 => "h4",
            61 => "a3", 62 => "b3", 63 => "c3", 64 => "d3", 65 => "e3", 66 => "f3", 67 => "g3", 68 => "h3",
            71 => "a2", 72 => "b2", 73 => "c2", 74 => "d2", 75 => "e2", 76 => "f2", 77 => "g2", 78 => "h2",
            81 => "a1", 82 => "b1", 83 => "c1", 84 => "d1", 85 => "e1", 86 => "f1", 87 => "g1", 88 => "h1",
            _ => panic!("an illegal index was passed in: {}", index),
        };
        String::from(square)
    }

    pub fn generate_square_from_string(square: char) -> Square {
        match square {
            'p' => Square::new(Piece::new(PieceType::Pawn, Color::Black)),
            'P' => Square::new(Piece::new(PieceType::Pawn, Color::White)),
            'b' => Square::new(Piece::new(PieceType::Bishop, Color::Black)),
            'B' => Square::new(Piece::new(PieceType::Bishop, Color::White)),
            'n' => Square::new(Piece::new(PieceType::Knight, Color::Black)),
            'N' => Square::new(Piece::new(PieceType::Knight, Color::White)),
            'r' => Square::new(Piece::new(PieceType::Rook, Color::Black)),
            'R' => Square::new(Piece::new(PieceType::Rook, Color::White)),
            'q' => Square::new(Piece::new(PieceType::Queen, Color::Black)),
            'Q' => Square::new(Piece::new(PieceType::Queen, Color::White)),
            'k' => Square::new(Piece::new(PieceType::King, Color::Black)),
            'K' => Square::new(Piece::new(PieceType::King, Color::White)),
            '0' => Square{ piece: None, is_edge: true },
            '-' => Square{ piece: None, is_edge: false },
            _ => panic!("Received piece char other than accepted values")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod index_to_square {
        use super::*;

        #[test]
        #[should_panic]
        fn it_panics_with_unknown_square_input() {
            helpers::index_to_square(10);
        }

        #[test]
        fn it_converts_index_to_chess_square() {
            let index = helpers::index_to_square(11);
            assert_eq!(index, String::from("a8"));
        }
    }

    mod square_to_index {
        use super::*;

        #[test]
        #[should_panic]
        fn it_panics_with_unknown_square_input() {
            helpers::square_to_index(String::from("a0"));
        }

        #[test]
        fn it_converts_chess_square_to_index() {
            let index = helpers::square_to_index(String::from("a1"));
            assert_eq!(index, 81);
        }

        #[test]
        fn it_is_case_insensitive() {
            let index = helpers::square_to_index(String::from("A1"));
            assert_eq!(index, 81);
        }
    }

    mod generate_square_from_string {
        use super::*;

        #[test]
        #[should_panic]
        fn it_panics_with_unrecognized_piece() {
            let square_char = "z".chars().next().unwrap();
            helpers::generate_square_from_string(square_char);
        }

        #[test]
        fn it_returns_square_struct() {
            let square_char = "r".chars().next().unwrap();
            let square: Square = helpers::generate_square_from_string(square_char);
            match square.piece {
                Some(p) => assert_eq!(p.color, Color::Black),
                None => panic!("Expected Piece not found"),
            }
        }

        #[test]
        fn it_returns_squares_with_proper_color_enum() {
            let square_char = "R".chars().next().unwrap();
            let square: Square = helpers::generate_square_from_string(square_char);
            match square.piece {
                Some(p) => assert_eq!(p, Piece { piece_type: PieceType::Rook, color: Color::White }),
                None => panic!("Expected Piece not found"),
            }
        }
    }

    mod generate_board {
        use super::*;
        
        #[test]
        #[should_panic]
        fn it_panics_when_board_is_missing_square() {
            let board_string_missing_one = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR0000000000");
            Board::new(board_string_missing_one, Color::White);
        }

        #[test]
        #[should_panic]
        fn it_panics_when_board_has_too_many_squares() {
            let board_string_missing_one = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR000000000000");
            Board::new(board_string_missing_one, Color::White);
        }

        #[test]
        #[should_panic]
        fn it_panics_when_board_has_unknown_piece() {
            let board_string_missing_one = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNzQKBNR0000000000");
            Board::new(board_string_missing_one, Color::White);
        }

        #[test]
        fn it_generates_board_with_proper_inputs() {
            let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
            let board: Board = Board::new(board_string, Color::White);
            assert_eq!(board.squares.len(), 100);
        }
    }

    mod move_struct {
        use super::*;
        
        mod from_chess_move {
            use super::*;

            #[test]
            fn it_returns_a_move_struct_when_given_chess_moves() {
                let chess_move = Move::from_chess_move((String::from("E2"), String::from("E4")));
                assert_eq!(chess_move.from, 75);
                assert_eq!(chess_move.to, 55);
            }
        }

        mod to_chess_move {
            use super::*;

            #[test]
            fn it_returns_a_tuple_of_chess_moves() {
                let move_struct = Move { from: 75, to: 55 };
                let (from, to) = move_struct.to_chess_move();
                assert_eq!(from, "e2");
                assert_eq!(to, "e4");
            }
        }
    }

    mod board_struct {
        use super::*;

        mod set_square {
            use super::*;

            #[test]
            #[should_panic]
            fn it_panics_if_move_involves_invalid_square() {
                let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
                let mut board: Board = Board::new(board_string, Color::White);
                board.set_square(101, None);
            }

            #[test]
            fn it_sets_the_target_index_to_piece() {
                let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
                let mut board: Board = Board::new(board_string, Color::White);
                assert!(match board.get_piece_at(15) {
                    Some(p) => p == Piece { piece_type: PieceType::King, color: Color::Black },
                    None => false,
                });

                board.set_square(15, Some(Piece::new(PieceType::King, Color::White)));
                assert!(match board.get_piece_at(15) {
                    Some(p) => p == Piece { piece_type: PieceType::King, color: Color::White },
                    None => false,
                });

                board.set_square(15, Some(Piece::new(PieceType::King, Color::Black)));
                assert!(match board.get_piece_at(15) {
                    Some(p) => p == Piece { piece_type: PieceType::King, color: Color::Black },
                    None => false,
                });
  
                board.set_square(15, Some(Piece::new(PieceType::King, Color::White)));
                assert!(match board.get_piece_at(15) {
                    Some(p) => p == Piece { piece_type: PieceType::King, color: Color::White },
                    None => false,
                });

                board.set_square(15, Some(Piece::new(PieceType::King, Color::Black)));
                assert!(match board.get_piece_at(15) {
                    Some(p) => p == Piece { piece_type: PieceType::King, color: Color::Black },
                    None => false,
                });

                board.set_square(15, Some(Piece::new(PieceType::King, Color::White)));
                assert!(match board.get_piece_at(15) {
                    Some(p) => p == Piece { piece_type: PieceType::King, color: Color::White },
                    None => false,
                });
            }
        }

        mod make_move {
            use super::*;

            #[test]
            #[should_panic]
            fn it_panics_if_moves_involve_invalid_squares() {
                let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
                let mut board: Board = Board::new(board_string, Color::White);
                board.make_move(Move { from: 75, to: 150 });                
            }

            #[test]
            #[should_panic]
            fn it_panics_if_from_square_is_not_a_piece() {
                let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
                let mut board: Board = Board::new(board_string, Color::White);
                board.make_move(Move { from: 65, to: 55 });
            }

            #[test]
            fn the_from_square_has_none_piece_after_move() {
                let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
                let mut board: Board = Board::new(board_string, Color::White);
                board.make_move(Move { from: 75, to: 55 });
                assert_eq!(board.get_piece_at(75), None);
            }

            #[test]
            fn the_to_square_has_the_correct_piece_after_move() {
                let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
                let mut board: Board = Board::new(board_string, Color::White);
                board.make_move(Move { from: 75, to: 55 });
                assert!(match board.get_piece_at(55) {
                    Some(p) => p == Piece { piece_type: PieceType::Pawn, color: Color::White },
                    None => false,
                })
            }

            #[test]
            fn it_toggles_current_turn_after_each_move() {
                let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
                let mut board: Board = Board::new(board_string, Color::White);
                assert_eq!(board.current_turn.color, Color::White);
                board.make_move(Move { from: 75, to: 55 });
                assert_eq!(board.current_turn.color, Color::Black);
                board.make_move(Move { from: 25, to: 45 });
                assert_eq!(board.current_turn.color, Color::White);
                board.make_move(Move { from: 74, to: 54 });
                assert_eq!(board.current_turn.color, Color::Black);
                board.make_move(Move { from: 24, to: 44 });
                assert_eq!(board.current_turn.color, Color::White);
                board.make_move(Move { from: 73, to: 53 });
                assert_eq!(board.current_turn.color, Color::Black);
            }
        }
        
        mod test_move {
            use super::*;

            #[test]
            fn it_returns_a_new_board_with_proper_squares() {
                let board_string = String::from("00000000000rnbqkbnr00pppppppp00--------00--------00--------00--------00PPPPPPPP00RNBQKBNR00000000000");
                let board: Board = Board::new(board_string, Color::White);
                let tested_board: Board = board.test_move(Move { from: 75, to: 55 });
                assert!(match board.get_piece_at(75) {
                    Some(p) => p.piece_type == PieceType::Pawn,
                    None => false,
                });

                assert!(match tested_board.get_piece_at(75) {
                    Some(_) => false,
                    None => true,
                });
            }
        }
    }
}