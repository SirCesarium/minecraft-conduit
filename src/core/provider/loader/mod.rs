mod vanilla;
mod fabric;
mod forge;
mod neoforge;
mod paper;
mod purpur;

pub use vanilla::VanillaProvider;
pub use fabric::FabricProvider;
pub use forge::ForgeProvider;
pub use neoforge::NeoForgeProvider;
pub use paper::PaperProvider;
pub use purpur::PurpurProvider;

use std::cmp::Ordering;
use std::future::Future;

use minecraft_registry_api::error::ApiError;

pub trait VersionProvider {
    fn fetch_game_versions(&self, http: &reqwest::Client) -> impl Future<Output = Result<Vec<String>, ApiError>> + Send;
    fn fetch_loader_versions(
        &self,
        http: &reqwest::Client,
        game_version: &str,
    ) -> impl Future<Output = Result<Vec<String>, ApiError>> + Send;
}

use crate::core::model::loader::LoaderKind;

impl VersionProvider for LoaderKind {
    async fn fetch_game_versions(&self, http: &reqwest::Client) -> Result<Vec<String>, ApiError> {
        match self {
            Self::Vanilla => VanillaProvider.fetch_game_versions(http).await,
            Self::Fabric => FabricProvider.fetch_game_versions(http).await,
            Self::Forge => ForgeProvider.fetch_game_versions(http).await,
            Self::NeoForge => NeoForgeProvider.fetch_game_versions(http).await,
            Self::Paper => PaperProvider.fetch_game_versions(http).await,
            Self::Purpur => PurpurProvider.fetch_game_versions(http).await,
        }
    }

    async fn fetch_loader_versions(
        &self,
        http: &reqwest::Client,
        game_version: &str,
    ) -> Result<Vec<String>, ApiError> {
        match self {
            Self::Vanilla => VanillaProvider.fetch_loader_versions(http, game_version).await,
            Self::Fabric => FabricProvider.fetch_loader_versions(http, game_version).await,
            Self::Forge => ForgeProvider.fetch_loader_versions(http, game_version).await,
            Self::NeoForge => NeoForgeProvider.fetch_loader_versions(http, game_version).await,
            Self::Paper => PaperProvider.fetch_loader_versions(http, game_version).await,
            Self::Purpur => PurpurProvider.fetch_loader_versions(http, game_version).await,
        }
    }
}

fn is_stable_version(v: &str) -> bool {
    !v.contains("pre")
        && !v.contains("rc")
        && !v.contains("snapshot")
        && !v.contains("alpha")
        && !v.contains("beta")
}

fn is_valid_version(v: &str) -> bool {
    let parts: Vec<&str> = v.split('.').collect();
    if parts.len() < 2 || parts.len() > 3 {
        return false;
    }
    parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
}

fn cmp_versions(a: &str, b: &str) -> Ordering {
    fn parts(v: &str) -> impl Iterator<Item = u32> + '_ {
        v.split('.').filter_map(|s| s.parse().ok())
    }
    parts(a).cmp(parts(b))
}
