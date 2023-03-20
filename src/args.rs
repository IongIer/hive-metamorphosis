use clap::{Parser, ValueEnum};

#[derive(Clone, Debug, ValueEnum)]
pub enum Mode {
    Uhp,
    Pgn,
    Sgf,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    #[clap(
        short = 'p',
        long = "path",
        required = true,
        value_name = "PATH/TO/FILE"
    )]
    pub path: String,
    #[clap(
        short = 'm',
        long = "mode",
        required = true,
        value_enum,
        value_name = "Conversion mode"
    )]
    pub mode: Mode,
    #[clap(short = 'v', long = "verbose", value_name = "Verbose output")]
    pub verbose: bool,
}
