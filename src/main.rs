mod args;
mod pgn_to_uhp;

use args::CliArgs;
use clap::Parser;
use pgn_to_uhp::create_output_uhp_string;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let cli_args = CliArgs::parse();
    let file_path = Path::new(&cli_args.path);
    let _verbose = &cli_args.verbose;
    match &cli_args.mode {
        args::Mode::Uhp => create_output_uhp_string(file_path)?,
        args::Mode::Pgn => {}
        args::Mode::Sgf => {}
    }
    Ok(())
}
