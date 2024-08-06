use colored::Colorize;
use structs::{Game, Square, Team};
mod structs;
mod utils;

fn is_legal_move(
    game: &mut Game,
    is_white_turn: bool,
    start_sq: &Square,
    target_sq: &Square,
) -> Result<(), String> {
    let start_piece = game.get_board()[start_sq.get_rank()][start_sq.get_file()].clone();
    let target_piece = game.get_board()[target_sq.get_rank()][target_sq.get_file()].clone();

    if let None = start_piece {
        return Err("There is no piece there.".to_string());
    }

    let start_peice = start_piece.unwrap();

    match (&start_peice.get_team(), is_white_turn) {
        (&Team::Black, true) => return Err("That's not your piece.".to_string()),
        (&Team::White, false) => return Err("That's not your piece.".to_string()),
        _ => {}
    };

    if let Some(target_piece) = target_piece {
        if target_piece.get_team() == start_peice.get_team() {
            return Err("You cannot capture your own peice.".to_string());
        }
    }

    Ok(())
}

fn main() {
    let mut game = Game::init();

    let mut is_whites_turn = true;
    let mut error_message: Option<String> = None;

    let white_lost = loop {
        let mut input = String::new();
        game.display_board(is_whites_turn);
        if let Some(error_message) = error_message {
            println!(
                "{}\n",
                format!("Error: {error_message} Try Again.").red().bold()
            );
        }
        println!(
            "{}, your turn! Enter the coordinate of the piece you want to move followed by the coordinate of the target square",
            if is_whites_turn { "Blue" } else { "Red" }
        );

        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(err) => {
                error_message = Some(format!("Error reading input: {}", err));
                continue;
            }
        }
        input = input.trim().to_string();

        if input == "q" {
            break is_whites_turn;
        }

        let coords: Vec<&str> = input.split(" ").collect();

        if coords.len() != 2 {
            error_message = Some(format!("Error: Must input 2 coordinates"));
            continue;
        }

        let start_sq = Square::from_coords(coords[0]);
        let target_sq = Square::from_coords(coords[1]);

        let (start_sq, target_sq) = match (start_sq, target_sq) {
            (Ok(start_sq), Ok(target_sq)) => (start_sq, target_sq),
            _ => {
                error_message = Some(format!("Invalid Coordinates"));
                continue;
            }
        };

        if let Err(e) = game.move_piece(is_whites_turn, start_sq, target_sq) {
            error_message = Some(format!("{e}"));
            continue;
        } else {
            error_message = None;
            is_whites_turn = !is_whites_turn;
        }
    };

    let winner = if white_lost { "Black" } else { "White" };
    println!("\nCongratulations {}", winner);

    println!("Thanks for playing!!")
}
