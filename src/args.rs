use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    #[clap(short = 'p', long = "path", required = true, value_name = "PATH/TO/FILE")]
    pub path: String,
}
