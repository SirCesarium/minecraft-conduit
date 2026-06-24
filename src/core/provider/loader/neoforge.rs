use std::collections::BTreeSet;
use std::future::Future;

use minecraft_registry_api::error::ApiError;
use minecraft_registry_api::neoforge::NeoForgeClient;

use super::VersionProvider;

use super::{cmp_versions, is_valid_version};

use crate::core::model::addon::AddonKind;
use crate::core::provider::loader::AddonFolderProvider;

pub struct NeoForgeProvider;

impl VersionProvider for NeoForgeProvider {
    fn fetch_game_versions(&self, http: &reqwest::Client) -> impl Future<Output = Result<Vec<String>, ApiError>> + Send {
        let http = http.clone();
        async move {
            let neoforge = NeoForgeClient::new(http);
            let meta = neoforge.get_metadata().await?;
            let mut seen = BTreeSet::new();
            for v in &meta.versioning.versions.list {
                let parts: Vec<&str> = v.splitn(3, '.').collect();
                if let [major_minor, patch] = &parts[..2] {
                    let ver = format!("1.{major_minor}.{patch}");
                    if is_valid_version(&ver) {
                        seen.insert(ver);
                    }
                }
            }
            let mut versions: Vec<String> = seen.into_iter().collect();
            versions.sort_by(|a, b| cmp_versions(b, a));
            Ok(versions)
        }
    }

    fn fetch_loader_versions(
        &self,
        http: &reqwest::Client,
        game_version: &str,
    ) -> impl Future<Output = Result<Vec<String>, ApiError>> + Send {
        let http = http.clone();
        let game_version = game_version.to_string();
        async move {
            let neoforge = NeoForgeClient::new(http);
            let meta = neoforge.get_metadata().await?;
            let target = game_version
                .strip_prefix("1.")
                .and_then(|s| s.split_once('.'))
                .map(|(a, b)| format!("{a}.{b}"))
                .unwrap_or_default();
            let mut versions: Vec<String> = meta
                .versioning
                .versions
                .list
                .into_iter()
                .filter(|v| {
                    let parts: Vec<&str> = v.splitn(3, '.').collect();
                    !v.contains("beta")
                        && parts.len() >= 2
                        && format!("{}.{}", parts[0], parts[1]) == target
                })
                .collect();
            versions.sort_by(|a, b| cmp_versions(b, a));
            Ok(versions)
        }
    }
}

impl AddonFolderProvider for NeoForgeProvider {
    fn get_addon_folder(&self, r#type: AddonKind) -> Option<&'static str> {
        match r#type {
            AddonKind::Mod => Some("mods"),
            AddonKind::Plugin => None,
        }
    }
}
