use std::path::PathBuf;

use clap::Args;
use miette::IntoDiagnostic;
use minecraft_registry_api::mojang::MojangClient;

use crate::{
    cli::Runnable,
    core::schema::{constants::MANIFEST_NAME, manifest::ConduitManifest},
    info,
};

#[derive(Args)]
pub struct Cmd {
    #[arg(default_value = ".")]
    dir: PathBuf,
}

impl Runnable for Cmd {
    async fn run(self) -> miette::Result<()> {
        info!("Initializing...");

        let http_client = reqwest::Client::builder()
            .user_agent("minecraft-conduit/0.1.0")
            .build()
            .into_diagnostic()?;

        let mojang = MojangClient::new(http_client);

        let version_manifest = mojang.get_manifest().await.into_diagnostic()?;

        let manifest_path = self.dir.join(MANIFEST_NAME);

        let mut manifest = ConduitManifest::load(&manifest_path)
            .await
            .unwrap_or_default();

        manifest.loader.game_version = Some(version_manifest.latest.release);

        let dir_name = self
            .dir
            .canonicalize()
            .into_diagnostic()?
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default();
        manifest.name = dir_name;

        manifest.save(&manifest_path).await.into_diagnostic()?;

        Ok(())
    }
}
