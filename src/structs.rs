use crate::{
    is_legal_move,
    utils::{get_default_board, get_piece_char},
};

pub struct Game {
    board: [[Option<Piece>; 8]; 8],
}
impl Game {
    pub fn init() -> Game {
        Game {
            board: get_default_board(),
        }
    }
    pub fn get_board(&self) -> &[[Option<Piece>; 8]; 8] {
        &self.board
    }
    pub fn move_piece(
        &mut self,
        is_white_turn: bool,
        start_sq: Square,
        target_sq: Square,
    ) -> Result<(), String> {
        if let Err(e) = is_legal_move(self, is_white_turn, &start_sq, &target_sq) {
            return Err(e);
        }

        let piece = self.board[start_sq.rank][start_sq.file].clone();

        self.board[start_sq.rank][start_sq.file] = None;
        self.board[target_sq.rank][target_sq.file] = piece;

        Ok(())
    }
    pub fn display_board(&self, is_whites_turn: bool) {
        println!("   -----------------------------------------");
        let mut board = self.board.clone();

        if is_whites_turn {
            board.reverse();
        }

        for (index, rank) in board.iter().enumerate() {
            if is_whites_turn {
                print!("{}  ", 8 - index);
            } else {
                print!("{}  ", index + 1);
            }
            let mut rank = rank.to_owned();
            if !is_whites_turn {
                rank.reverse();
            }
            for square in rank.iter() {
                match square {
                    Some(square) => print!("| {}  ", get_piece_char(square.clone())),
                    None => print!("|    "),
                }
            }
            println!("|");
            println!("   -----------------------------------------");
        }
        if is_whites_turn {
            println!("     a    b    c    d    e    f    g    h\n")
        } else {
            println!("     h    g    f    e    d    c    b    a\n")
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Team {
    White,
    Black,
}
#[derive(Debug, Clone)]
pub struct Piece {
    piece_type: PieceType,
    team: Team,
}
impl Piece {
    pub fn new(piece_type: PieceType, team: Team) -> Piece {
        Piece { piece_type, team }
    }
    pub fn get_team(&self) -> &Team {
        &self.team
    }
    pub fn get_piece_type(&self) -> &PieceType {
        &self.piece_type
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Debug)]
pub struct Square {
    rank: usize,
    file: usize,
}
impl Square {
    pub fn new(rank: usize, file: usize) -> Result<Square, String> {
        if rank > 7 || file > 7 {
            return Err("Error: Invalid Square".to_string());
        };
        return Ok(Square { rank, file });
    }
    pub fn get_rank(&self) -> usize {
        self.rank
    }
    pub fn get_file(&self) -> usize {
        self.file
    }
    pub fn from_coords(coords: &str) -> Result<Square, String> {
        if coords.chars().count() != 2 {
            return Err("Error: Invalid Coordinates".to_string());
        }
        let mut chars = coords.chars();
        let file_str = chars.next().unwrap();
        let rank_str = chars.next().unwrap();

        let file_strs = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let file = match file_strs.iter().position(|&ele| ele == file_str) {
            Some(file) => file,
            None => return Err("Error: Invalid Coordinates".to_string()),
        };

        let rank: usize = match rank_str.to_string().parse() {
            Ok(rank) => rank,
            Err(_) => return Err("Error: Invalid Coordinates".to_string()),
        };
        let rank = rank - 1;

        Square::new(rank, file)
    }
}
