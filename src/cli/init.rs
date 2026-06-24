use std::path::PathBuf;
use clap::Args;
use miette::IntoDiagnostic;
use minecraft_registry_api::mojang::MojangClient;
use crate::{
    cli::Runnable,
    core::model::loader::LoaderKind,
    display::prompts,
    func::init::init,
    utils::{build_http_client, get_dir_name},
};

#[derive(Args)]
pub struct Cmd {
    #[arg(default_value = ".")]
    dir: PathBuf,
    #[arg(short, long)]
    yes: bool,
}

impl Runnable for Cmd {
    async fn run(self) -> miette::Result<()> {
        let http = build_http_client().into_diagnostic()?;
        let name = get_dir_name(&self.dir).into_diagnostic()?;

        if self.yes {
            return run_with_defaults(&http, self.dir, name).await;
        }

        prompts::init::run_interactive(&http, self.dir, name).await
    }
}

async fn run_with_defaults(
    http: &reqwest::Client,
    dir: PathBuf,
    name: String,
) -> miette::Result<()> {
    let mojang = MojangClient::new(http.clone());
    let manifest = mojang.get_manifest().await.into_diagnostic()?;

    init(dir, name, manifest.latest.release, LoaderKind::Vanilla, None)
        .await
        .into_diagnostic()
}
