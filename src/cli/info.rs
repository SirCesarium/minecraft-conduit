use clap::Args;

use crate::cli::Runnable;
use crate::{info, warn};
use crate::display::palette;
use crate::func::info::fetch_info;
use crate::utils::fmt::{date, loaders, number, versions};

#[derive(Args)]
pub struct Cmd {
    slug: String,
}

impl Runnable for Cmd {
    async fn run(self) -> miette::Result<()> {
        info!("Fetching info for \"{}\"", self.slug);

        let (project, sorted_versions) = match fetch_info(&self.slug).await {
            Ok(v) => v,
            Err(e) => {
                warn!("Project \"{}\" not found: {e}", self.slug);
                return Ok(());
            }
        };

        println!();
        println!(
            "{bold}{aqua}{}{reset}",
            project.title,
            bold = palette::BOLD,
            aqua = palette::FG_BRIGHT_AQUA,
            reset = palette::RESET,
        );
        println!(
            "{gray}  {} │ {}{reset}",
            project.slug,
            project.project_type,
            gray = palette::FG_GRAY,
            reset = palette::RESET,
        );

        if !project.description.is_empty() {
            println!(
                "\n{fg}{}{reset}",
                project.description,
                fg = palette::FG_FG,
                reset = palette::RESET,
            );
        }

        println!(
            "\n{bold}{blue}── Metadata ──{reset}",
            bold = palette::BOLD,
            blue = palette::FG_BRIGHT_BLUE,
            reset = palette::RESET,
        );

        println!(
            "{gray}  Downloads:{reset}        {orange}{}{reset}",
            number(project.downloads),
            gray = palette::FG_GRAY,
            reset = palette::RESET,
            orange = palette::FG_ORANGE,
        );
        println!(
            "{gray}  Followers:{reset}         {}",
            number(project.followers),
            gray = palette::FG_GRAY,
            reset = palette::RESET,
        );
        println!(
            "{gray}  Status:{reset}            {}{reset}",
            project.status,
            gray = palette::FG_GRAY,
            reset = palette::RESET,
        );
        let license = if !project.license.name.is_empty() {
            &project.license.name
        } else if !project.license.id.is_empty() {
            &project.license.id
        } else {
            "—"
        };
        println!(
            "{gray}  License:{reset}           {license}",
            gray = palette::FG_GRAY,
            reset = palette::RESET,
        );
        println!(
            "{gray}  Client side:{reset}       {}",
            project.client_side,
            gray = palette::FG_GRAY,
            reset = palette::RESET,
        );
        println!(
            "{gray}  Server side:{reset}       {}",
            project.server_side,
            gray = palette::FG_GRAY,
            reset = palette::RESET,
        );
        println!(
            "{gray}  Published:{reset}         {}",
            date(&project.published),
            gray = palette::FG_GRAY,
            reset = palette::RESET,
        );
        println!(
            "{gray}  Updated:{reset}           {}",
            date(&project.updated),
            gray = palette::FG_GRAY,
            reset = palette::RESET,
        );

        if !project.loaders.is_empty() {
            println!(
                "{gray}  Loaders:{reset}           {}",
                loaders(&project.loaders),
                gray = palette::FG_GRAY,
                reset = palette::RESET,
            );
        }

        if !project.game_versions.is_empty() {
            println!(
                "{gray}  Game versions:{reset}     {}",
                versions(&project.game_versions, 8),
                gray = palette::FG_GRAY,
                reset = palette::RESET,
            );
        }

        if !project.categories.is_empty() {
            println!(
                "{gray}  Categories:{reset}        {}",
                project.categories.join(", "),
                gray = palette::FG_GRAY,
                reset = palette::RESET,
            );
        }

        println!(
            "\n{bold}{blue}── Links ──{reset}",
            bold = palette::BOLD,
            blue = palette::FG_BRIGHT_BLUE,
            reset = palette::RESET,
        );

        println!(
            "  {blue}Modrinth{reset}       {gray}https://modrinth.com/mod/{}{reset}",
            project.slug,
            blue = palette::FG_BLUE,
            reset = palette::RESET,
            gray = palette::FG_GRAY,
        );
        if let Some(url) = &project.issues_url
            && !url.is_empty()
        {
            println!(
                "  {blue}Issues{reset}         {gray}{url}{reset}",
                blue = palette::FG_BLUE,
                reset = palette::RESET,
                gray = palette::FG_GRAY,
            );
        }
        if let Some(url) = &project.source_url
            && !url.is_empty()
        {
            println!(
                "  {blue}Source{reset}         {gray}{url}{reset}",
                blue = palette::FG_BLUE,
                reset = palette::RESET,
                gray = palette::FG_GRAY,
            );
        }
        if let Some(url) = &project.wiki_url
            && !url.is_empty()
        {
            println!(
                "  {blue}Wiki{reset}           {gray}{url}{reset}",
                blue = palette::FG_BLUE,
                reset = palette::RESET,
                gray = palette::FG_GRAY,
            );
        }
        if let Some(url) = &project.discord_url
            && !url.is_empty()
        {
            println!(
                "  {blue}Discord{reset}        {gray}{url}{reset}",
                blue = palette::FG_BLUE,
                reset = palette::RESET,
                gray = palette::FG_GRAY,
            );
        }

        if !sorted_versions.is_empty() {
            println!(
                "\n{bold}{blue}── Recent Versions ──{reset}",
                bold = palette::BOLD,
                blue = palette::FG_BRIGHT_BLUE,
                reset = palette::RESET,
            );

            for v in sorted_versions.iter().take(5) {
                let version_type_tag = match v.version_type.as_str() {
                    "release" => format!("{}release{}", palette::FG_GREEN, palette::RESET),
                    "beta" => format!("{}beta{}", palette::FG_YELLOW, palette::RESET),
                    "alpha" => format!("{}alpha{}", palette::FG_RED, palette::RESET),
                    _ => format!("{}{}{}", palette::FG_GRAY, v.version_type, palette::RESET),
                };

                println!(
                    "  {bold}{name:<32}{reset} {tag}  {gray}{date}{reset}",
                    bold = palette::BOLD,
                    name = v.name,
                    reset = palette::RESET,
                    tag = version_type_tag,
                    gray = palette::FG_GRAY,
                    date = date(&v.date_published),
                );
                println!(
                    "  {gray}  {} v{}{reset}",
                    v.game_versions.join(", "),
                    v.version_number,
                    gray = palette::FG_GRAY,
                    reset = palette::RESET,
                );
                println!();
            }

            if sorted_versions.len() > 5 {
                println!(
                    "  {gray}... and {} more versions{reset}",
                    sorted_versions.len() - 5,
                    gray = palette::FG_GRAY,
                    reset = palette::RESET,
                );
            }
        }

        Ok(())
    }
}
