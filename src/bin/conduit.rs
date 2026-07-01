use clap::Parser;
use conduit::cli::{Cli, Command, Runnable};

#[tokio::main]
async fn main() -> miette::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Hello(cmd) => cmd.run().await,
        Command::Info(cmd) => cmd.run().await,
        Command::Init(cmd) => cmd.run().await,
        Command::Search(cmd) => cmd.run().await,
    }
}
