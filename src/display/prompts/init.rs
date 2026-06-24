use crate::{
    core::{model::loader::LoaderKind, provider::loader::VersionProvider},
    display::inquire,
    func::init::init,
};
use miette::{IntoDiagnostic, Result};
use std::path::PathBuf;
use strum::IntoEnumIterator;

pub(crate) async fn run_interactive(
    http: &reqwest::Client,
    dir: PathBuf,
    name: String,
) -> Result<()> {
    // Confirm server name and loader with the user
    let name = inquire::text_with_default("Server name", &name).into_diagnostic()?;
    let loader = inquire::select("Loader", LoaderKind::iter().collect()).into_diagnostic()?;

    // Chained selection: game version → loader build
    let game_version = pick_game_version(http, &loader).await?;
    let build = pick_loader_build(http, &loader, &game_version).await?;

    let result = init(dir, name, game_version, loader, build).await;

    info!("Project initialized");
    result.into_diagnostic()
}

/// Fetches available game versions for the chosen loader
/// and prompts the user to select one.
async fn pick_game_version(http: &reqwest::Client, loader: &LoaderKind) -> Result<String> {
    let versions = loader.fetch_game_versions(http).await.into_diagnostic()?;

    inquire::select("Game version", versions).into_diagnostic()
}

/// Fetches loader versions for the chosen game version
/// and prompts the user to select a build (if applicable).
async fn pick_loader_build(
    http: &reqwest::Client,
    loader: &LoaderKind,
    game_version: &str,
) -> Result<Option<String>> {
    let loader_versions = loader
        .fetch_loader_versions(http, game_version)
        .await
        .into_diagnostic()?;

    match loader {
        LoaderKind::Fabric => pick_fabric_build(loader_versions),
        _ => pick_generic_build(loader_versions),
    }
}

/// For Fabric, offers "Latest (recommended)" as the first option.
/// Returns Some(version) if the user picks a specific one, None otherwise.
fn pick_fabric_build(loader_versions: Vec<String>) -> Result<Option<String>> {
    use inquire::select;

    const LATEST: &str = "Latest (recommended)";

    let mut options = vec![LATEST.to_string()];
    options.extend(loader_versions);

    let pick = select("Loader version", options).into_diagnostic()?;

    // None = use latest, Some(ver) = pinned version
    Ok((pick != LATEST).then_some(pick))
}

/// For other loaders: uses the only version if there's one,
/// prompts if there are many, and skips if there are none.
fn pick_generic_build(loader_versions: Vec<String>) -> Result<Option<String>> {
    use inquire::select;

    match loader_versions.len() {
        0 => Ok(None),
        1 => Ok(loader_versions.into_iter().next()),
        _ => select("Loader version", loader_versions)
            .into_diagnostic()
            .map(Some),
    }
}
