use colored::ColoredString;
use colored::Colorize;

use crate::structs::Piece;
use crate::structs::PieceType;
use crate::structs::Team;

pub fn get_piece_char(piece: Piece) -> ColoredString {
    match (piece.get_team(), piece.get_piece_type()) {
        (Team::White, PieceType::King) => "♚".blue(),
        (Team::White, PieceType::Queen) => "♛".blue(),
        (Team::White, PieceType::Rook) => "♜".blue(),
        (Team::White, PieceType::Bishop) => "♝".blue(),
        (Team::White, PieceType::Knight) => "♞".blue(),
        (Team::White, PieceType::Pawn) => "♟".blue(),
        (Team::Black, PieceType::King) => "♚".red(),
        (Team::Black, PieceType::Queen) => "♛".red(),
        (Team::Black, PieceType::Rook) => "♜".red(),
        (Team::Black, PieceType::Bishop) => "♝".red(),
        (Team::Black, PieceType::Knight) => "♞".red(),
        (Team::Black, PieceType::Pawn) => "♟".red(),
    }
}

pub fn get_default_board() -> [[Option<Piece>; 8]; 8] {
    [
        [
            Some(Piece::new(PieceType::Rook, Team::White)),
            Some(Piece::new(PieceType::Knight, Team::White)),
            Some(Piece::new(PieceType::Bishop, Team::White)),
            Some(Piece::new(PieceType::Queen, Team::White)),
            Some(Piece::new(PieceType::King, Team::White)),
            Some(Piece::new(PieceType::Bishop, Team::White)),
            Some(Piece::new(PieceType::Knight, Team::White)),
            Some(Piece::new(PieceType::Rook, Team::White)),
        ],
        [
            Some(Piece::new(PieceType::Pawn, Team::White)),
            Some(Piece::new(PieceType::Pawn, Team::White)),
            Some(Piece::new(PieceType::Pawn, Team::White)),
            Some(Piece::new(PieceType::Pawn, Team::White)),
            Some(Piece::new(PieceType::Pawn, Team::White)),
            Some(Piece::new(PieceType::Pawn, Team::White)),
            Some(Piece::new(PieceType::Pawn, Team::White)),
            Some(Piece::new(PieceType::Pawn, Team::White)),
        ],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [
            Some(Piece::new(PieceType::Pawn, Team::Black)),
            Some(Piece::new(PieceType::Pawn, Team::Black)),
            Some(Piece::new(PieceType::Pawn, Team::Black)),
            Some(Piece::new(PieceType::Pawn, Team::Black)),
            Some(Piece::new(PieceType::Pawn, Team::Black)),
            Some(Piece::new(PieceType::Pawn, Team::Black)),
            Some(Piece::new(PieceType::Pawn, Team::Black)),
            Some(Piece::new(PieceType::Pawn, Team::Black)),
        ],
        [
            Some(Piece::new(PieceType::Rook, Team::Black)),
            Some(Piece::new(PieceType::Knight, Team::Black)),
            Some(Piece::new(PieceType::Bishop, Team::Black)),
            Some(Piece::new(PieceType::Queen, Team::Black)),
            Some(Piece::new(PieceType::King, Team::Black)),
            Some(Piece::new(PieceType::Bishop, Team::Black)),
            Some(Piece::new(PieceType::Knight, Team::Black)),
            Some(Piece::new(PieceType::Rook, Team::Black)),
        ],
    ]
}
