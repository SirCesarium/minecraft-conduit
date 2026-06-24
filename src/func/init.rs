use std::path::PathBuf;

use crate::{core::{model::loader::LoaderKind, schema::{constants::MANIFEST_NAME, manifest::ConduitManifest}}, errors::Error};

pub async fn init(
    dir: PathBuf,
    name: String,
    game_version: String,
    loader: LoaderKind,
    build: Option<String>,
) -> Result<(), Error> {
    let manifest_path = dir.join(MANIFEST_NAME);

    let mut manifest = ConduitManifest::load(&manifest_path)
        .await
        .unwrap_or_default();

    manifest.name = name;
    manifest.loader.kind = loader;
    manifest.loader.game_version = Some(game_version);
    manifest.loader.build = build;

    manifest.save(&manifest_path).await?;

    Ok(())
}
