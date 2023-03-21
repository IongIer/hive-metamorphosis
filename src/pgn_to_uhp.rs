use std::{
    fs::{self, File},
    io::{self, BufRead},
    path::Path,
};

use regex::Regex;

pub fn create_output_uhp_string(file_path: &Path, verbose: bool) -> std::io::Result<()> {
    let file_stem = file_path.file_stem().unwrap();
    let parent_dir = file_path.parent().unwrap();
    let mut new_file_path = parent_dir.to_path_buf();
    new_file_path.push("uhp");
    fs::create_dir_all(&new_file_path)?;
    new_file_path.push(file_stem);
    new_file_path.set_extension("txt");
    let answer = from_pgn(file_path);
    if verbose {
        println!("{answer}")
    }
    fs::write(new_file_path, answer)
}

fn from_pgn(file_path: &Path) -> String {
    // lines starting with a number are moves and the () capture the move
    let turn = Regex::new(r"^\d+\. (.*)").expect("This regex should compile");

    // captures the gametype out of line starting with [GameType
    let game_type =
        Regex::new(r#"\[GameType "(Base[+MLP]*)"\]"#).expect("This regex should compile");

    let mut extract_gametype = String::new();
    let mut moves = String::new();
    let result = ";InProgress";
    let mut pgn_move_number_number: u8 = 0;
    match File::open(file_path) {
        Ok(file) => {
            for line in io::BufReader::new(file).lines().flatten() {
                if line.is_empty() {
                    continue;
                }
                if extract_gametype.is_empty() && game_type.is_match(&line) {
                    let caps = game_type.captures(&line).unwrap();
                    if let Some(mtch) = caps.get(1) {
                        extract_gametype += mtch.as_str();
                    }
                }

                if turn.is_match(&line) {
                    let caps = turn.captures(&line).unwrap();
                    if let Some(mtch) = caps.get(1) {
                        moves += ";";
                        moves += mtch.as_str();
                        pgn_move_number_number += 1;
                    }
                } else {
                    continue;
                }
            }
        }
        Err(e) => {
            println!("Couldn't open file because: {e}");
        }
    }

    let move_number = pgn_move_number_number / 2;
    // pgn_move_number_number & 1 == 1 is a bitwise way of checking for odd/even
    // equivalent to pgn_move_number_number % 2 == 1
    let to_move = if pgn_move_number_number & 1 == 1 {
        format!(";Black[{move_number}]")
    } else {
        format!(";White[{move_number}]")
    };
    let answer = "newgame ".to_string()
        + extract_gametype.as_str()
        + result
        + to_move.as_str()
        + moves.as_str();
    answer
}
