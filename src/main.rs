mod pgn_to_uhp;

use pgn_to_uhp::create_output_file;
use std::env;

fn main() -> std::io::Result<()> {
    let cli_args: Vec<String> = env::args().collect();
    if let Some(file_path) = cli_args.get(1) {
        create_output_file(file_path)?
    } else {
        println!("No pgn provided");
    }
    Ok(())
}
