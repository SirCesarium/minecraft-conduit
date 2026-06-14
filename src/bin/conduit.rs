use clap::Parser;
use conduit::cli::{Cli, Command, Runnable};

#[tokio::main]
async fn main() -> miette::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Hello(cmd) => cmd.run().await,
    }
}
