use clap::Args;
use miette::IntoDiagnostic;

use crate::cli::Runnable;
use crate::warn;
use crate::display::palette;
use crate::func::search::{search, SearchInput};
use crate::utils::fmt::{downloads, title};

#[derive(Args)]
pub struct Cmd {
    query: String,

    #[arg(short, long)]
    loader: Option<String>,

    #[arg(short = 'g', long = "game-version")]
    game_version: Option<String>,

    #[arg(long = "loader-version")]
    loader_version: Option<String>,

    #[arg(short, long, default_value_t = 1)]
    page: usize,

    #[arg(short = 'n', long = "per-page", default_value_t = 20)]
    per_page: usize,
}

impl Runnable for Cmd {
    async fn run(self) -> miette::Result<()> {
        let output = search(SearchInput {
            query: &self.query,
            loader: self.loader.as_deref(),
            game_version: self.game_version.as_deref(),
            loader_version: self.loader_version.as_deref(),
            page: self.page,
            per_page: self.per_page,
        })
        .await
        .into_diagnostic()?;

        let shown = output.results.hits.len();
        let offset = (output.page - 1) * output.per_page;

        if output.results.total == 0 {
            warn!("No results found for \"{}\".", self.query);
            warn!("Try different search terms or fewer filters.");
            return Ok(());
        }

        println!(
            "{bold}{blue}── Results {start}–{end} of {total} (page {page}/{pages}) ──{reset}\n",
            bold = palette::BOLD,
            blue = palette::FG_BRIGHT_BLUE,
            start = offset + 1,
            end = offset + shown,
            total = output.results.total,
            page = self.page,
            pages = output.total_pages.max(1),
            reset = palette::RESET,
        );

        for (i, r) in output.results.hits.iter().enumerate() {
            println!(
                "{gray}{num:>4}.{reset} {bold}{aqua}{title}{reset}{gray}{dl:>15}{reset}",
                gray = palette::FG_GRAY,
                num = i + offset + 1,
                reset = palette::RESET,
                bold = palette::BOLD,
                aqua = palette::FG_BRIGHT_AQUA,
                title = title(&r.title, 50),
                dl = downloads(r.downloads),
            );
            println!(
                "     {blue}{slug:<30}{reset} {gray}https://modrinth.com/mod/{slug}{reset}",
                blue = palette::FG_BLUE,
                slug = r.slug,
                reset = palette::RESET,
                gray = palette::FG_GRAY,
            );
            if !r.description.is_empty() {
                let desc = if r.description.len() > 90 {
                    format!("{}...", &r.description[..87])
                } else {
                    r.description.clone()
                };
                println!("     {fg}{desc}{reset}", fg = palette::FG_FG, reset = palette::RESET);
            }
            println!();
        }

        if self.page < output.total_pages {
            let mut next_cmd = format!("conduit search \"{}\" --page {}", self.query, self.page + 1);
            if self.per_page != 20 {
                next_cmd.push_str(&format!(" --per-page {}", self.per_page));
            }
            if let Some(loader) = &self.loader {
                next_cmd.push_str(&format!(" --loader {loader}"));
            }
            if let Some(gv) = &self.game_version {
                next_cmd.push_str(&format!(" --game-version {gv}"));
            }

            println!(
                "{green}▶ Next page: {cmd}{reset}",
                green = palette::FG_GREEN,
                cmd = next_cmd,
                reset = palette::RESET,
            );
        }

        Ok(())
    }
}
