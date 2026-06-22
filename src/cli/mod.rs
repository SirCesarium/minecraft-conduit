pub mod hello;
pub mod init;

use clap::Parser;
use std::future::Future;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand)]
pub enum Command {
    Hello(hello::Cmd),
    Init(init::Cmd),
}

pub trait Runnable {
    fn run(self) -> impl Future<Output = miette::Result<()>> + Send;
}
