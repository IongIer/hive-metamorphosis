use regex::Regex;
use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    let cli_args: Vec<String> = env::args().collect();
    if let Some(file_path) = cli_args.get(1) {
        let answer = from_filepath(file_path);
        fs::write("out.txt", answer)?;
    } else {
        println!("No pgn provided");
    }
    Ok(())
}

fn from_filepath(file_path: &str) -> String {
    let turn = Regex::new(r"^\d+\. (.*)").expect("This regex should compile");
    let game_type =
        Regex::new(r#"\[GameType "(Base[+MLP]*)"\]"#).expect("This regex should compile");
    let result = Regex::new(r#"^\[Result "(.*)"\]"#).expect("This regex should compile");
    let mut extract_gametype = String::new();
    let mut moves = String::new();
    let mut extract_result = String::new();
    let mut pgn_move_number_number: u8 = 0;
    match File::open(file_path) {
        Ok(file) => {
            for line in io::BufReader::new(file).lines().flatten() {
                if line.len() == 0 {
                    continue;
                }
                if game_type.is_match(&line) {
                    let caps = game_type.captures(&line).unwrap();
                    if let Some(mtch) = caps.get(1) {
                        extract_gametype += mtch.as_str();
                    }
                }
                if result.is_match(&line) {
                    let caps = result.captures(&line).unwrap();
                    if let Some(mtch) = caps.get(1) {
                        extract_result += ";";
                        extract_result += mtch.as_str();
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
            println!("Couldn't open file because: {}", e);
        }
    }

    // pgn_move_number_number & 1 == 1 is a bitwise way of checking for odd/even
    // equivalent to pgn_move_number_number % 2 == 1
    let to_move = if pgn_move_number_number & 1 == 1 {
        format!(";Black[{}]", pgn_move_number_number / 2)
    } else {
        format!(";White[{}]", pgn_move_number_number / 2)
    };
    let answer = "newgame ".to_string()
        + extract_gametype.as_str()
        + extract_result.as_str()
        + to_move.as_str()
        + moves.as_str();
    return answer;
}
