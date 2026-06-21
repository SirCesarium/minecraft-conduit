use std::future::Future;
use std::sync::Arc;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use crate::display::palette;

fn template(s: &str) -> ProgressStyle {
    ProgressStyle::with_template(s).unwrap_or_else(|_| panic!("invalid template: {s:?}"))
}

pub fn new_progress_bar(len: u64) -> ProgressBar {
    let bar = ProgressBar::new(len);
    bar.set_style(
        template(palette::PROGRESS_BAR_TEMPLATE).progress_chars("█·"),
    );
    bar
}

pub fn new_multi() -> MultiProgress {
    MultiProgress::new()
}

pub fn add_progress_bar(mp: &MultiProgress, len: u64) -> ProgressBar {
    let bar = new_progress_bar(len);
    mp.add(bar)
}

pub fn finish_bar(bar: &ProgressBar, name: &str) {
    bar.set_style(template(palette::DONE_TEMPLATE));
    bar.finish_with_message(format!(
        "{fg}✓{reset} {name}",
        fg = palette::DONE_FG,
        reset = palette::RESET,
    ));
}

pub async fn run_parallel<T, F, Fut>(
    items: impl IntoIterator<Item = T>,
    max_concurrency: usize,
    f: F,
) -> miette::Result<()>
where
    T: Send + 'static,
    F: Fn(T) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = miette::Result<()>> + Send,
{
    let sem = Arc::new(Semaphore::new(max_concurrency));
    let f = Arc::new(f);

    let mut set = JoinSet::new();
    for item in items {
        let sem = Arc::clone(&sem);
        let f = Arc::clone(&f);
        set.spawn(async move {
            let _permit = sem.acquire().await.map_err(|e| miette::miette!("{e}"))?;
            f(item).await
        });
    }

    while let Some(res) = set.join_next().await {
        res.map_err(|e| miette::miette!("{e}"))??;
    }

    Ok(())
}