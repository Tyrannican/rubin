pub use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version)]
#[command(propagate_version = true)]
#[command(about = "Rubin CLI for client / server interactions", long_about = None)]
pub struct CliParser {
    #[clap(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Start a Rubin server on a given address / port
    Server(ConnectArgs),

    /// Start the CLI to interact with a Rubin server on a given address / port
    Cli(ConnectArgs),
}

#[derive(Args, Debug)]
pub struct ConnectArgs {
    /// Server address to use
    #[arg(short, long, default_value_t = String::from("127.0.0.1"))]
    pub address: String,

    /// Server port to use
    #[arg(short, long, default_value_t = 9876)]
    pub port: usize,
}
