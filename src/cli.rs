use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Args {
    /// Destination file to save to.
    /// If not provided, result will be printed to stdout.
    #[arg(short, long)]
    pub destination: Option<String>,

    /// Urls to get the public keys from
    #[arg(short, long, required=true, num_args = 1..)]
    pub sources: Vec<String>,

    /// Request timeout in seconds
    #[arg(short, long, default_value = "5")]
    pub timeout: u64,
}

pub fn parse_args() -> Args {
    Args::parse()
}
