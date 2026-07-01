pub mod hello;
pub mod info;
pub mod init;
pub mod search;

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
    Info(info::Cmd),
    Init(init::Cmd),
    Search(search::Cmd),
}

pub trait Runnable {
    fn run(self) -> impl Future<Output = miette::Result<()>> + Send;
}
