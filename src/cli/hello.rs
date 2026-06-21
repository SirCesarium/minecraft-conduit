use std::sync::Arc;
use std::time::Duration;

use clap::Args;
use tokio::time::sleep;

use crate::{
    cli::Runnable, display::{progress::{add_progress_bar, finish_bar, new_multi, run_parallel}, spinner::{finish_spinner, new_spinner}}, error, info, warn
};

#[derive(Args)]
pub struct Cmd {
    name: Option<String>,
}

impl Runnable for Cmd {
    async fn run(self) -> miette::Result<()> {
        let _name = self.name.unwrap_or_else(|| "world".into());

        let mp = Arc::new(new_multi());

        run_parallel(1..=30, 8, move |i| {
            let seed = i * 7 + 13;
            let size = (seed % 50 + 20) as u64;
            let delay_ms = (seed % 30 + 10) as u64;
            let mp = Arc::clone(&mp);
            async move {
                let bar = add_progress_bar(&mp, size);
                let name = format!("download {i}");
                bar.set_message(format!("downloading {name}"));
                for _ in 0..size {
                    sleep(Duration::from_millis(delay_ms)).await;
                    bar.inc(1);
                }
                finish_bar(&bar, &name);
                Ok(())
            }
        })
        .await?;

        let spinner = new_spinner("Post-processing...");
        for _ in 0..10 {
            sleep(Duration::from_millis(100)).await;
        }
        finish_spinner(&spinner, "Post-processing done");

        info!("Installed dependencies!");
        warn!("Installed dependencies!");
        error!("Installed dependencies!");
        Ok(())
    }
}
