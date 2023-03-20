mod args;
mod pgn_to_uhp;
use args::CliArgs;
use clap::Parser;
use pgn_to_uhp::create_output_uhp_string;
use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let cli_args = CliArgs::parse();
    let file_path = Path::new(&cli_args.path);
    let verbose = &cli_args.verbose;
    match &cli_args.mode {
        args::Mode::Uhp => {
            if let Some(ext) = file_path.extension() {
                if ext == "pgn" {
                    create_output_uhp_string(file_path, *verbose)?
                } else {
                    panic!("File is not a PGN");
                }
            } else {
                let files = fs::read_dir(file_path).unwrap();
                files
                    .filter_map(Result::ok)
                    .filter(|d| {
                        if let Some(ext) = d.path().extension() {
                            ext == "pgn"
                        } else {
                            false
                        }
                    })
                    .try_for_each(|f| create_output_uhp_string(f.path().as_ref(), *verbose))?;
            }
        }
        args::Mode::Pgn => {}
    }
    Ok(())
}
