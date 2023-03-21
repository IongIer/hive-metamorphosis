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
    let partial_closure_uhp: &dyn Fn(&Path) -> std::io::Result<()> = &|x| create_output_uhp_string(x, *verbose);    
    match &cli_args.mode {
        args::Mode::Uhp => {
            if let Some(ext) = file_path.extension() {
                if ext == "pgn" {
                    partial_closure_uhp(file_path)?
                } else {
                    panic!("File is not a PGN");
                }
            } else {
                process_many(file_path, "pgn" , partial_closure_uhp)
            }
        }
        args::Mode::Pgn => {}
    }
    Ok(())
}

fn process_many(file_path: &Path, extension: &str, func: &dyn Fn(&Path) -> std::io::Result<()> )  {
    let files = fs::read_dir(file_path).unwrap();
    files
        .filter_map(Result::ok)
        .filter(|dir_entry| {
            if let Some(ext) = dir_entry.path().extension() {
                ext == extension
            } else {
                false
            }
        })
        .try_for_each(|file_to_process| func(file_to_process.path().as_ref()));


}
