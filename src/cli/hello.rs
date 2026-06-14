use clap::Args;

use crate::cli::Runnable;

#[derive(Args)]
pub struct Cmd {
    name: Option<String>,
}

impl Runnable for Cmd {
    async fn run(self) -> miette::Result<()> {
        let name = self.name.unwrap_or_else(|| "world".into());
        println!("Hello, {name}!");
        Ok(())
    }
}
