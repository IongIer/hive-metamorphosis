mod args;
mod pgn_to_uhp;

use args::CliArgs;
use clap::Parser;
use pgn_to_uhp::create_output_file;



fn main() -> std::io::Result<()> {
    let cli_args = CliArgs::parse();
    let file_path = &cli_args.path;
    create_output_file(file_path)?;
    Ok(())
}
