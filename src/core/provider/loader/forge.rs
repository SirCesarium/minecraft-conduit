use std::collections::BTreeSet;
use std::future::Future;

use minecraft_registry_api::error::ApiError;
use minecraft_registry_api::forge::ForgeClient;

use super::VersionProvider;

use super::cmp_versions;

pub struct ForgeProvider;

impl VersionProvider for ForgeProvider {
    fn fetch_game_versions(&self, http: &reqwest::Client) -> impl Future<Output = Result<Vec<String>, ApiError>> + Send {
        let http = http.clone();
        async move {
            let forge = ForgeClient::new(http);
            let promos = forge.get_promos().await?;
            let mut versions: Vec<String> = promos
                .promos
                .keys()
                .filter_map(|k| k.split('-').next())
                .map(|s| s.to_string())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect();
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
            let forge = ForgeClient::new(http);
            let promos = forge.get_promos().await?;
            let mut versions = Vec::new();
            for (key, value) in &promos.promos {
                if key.starts_with(&format!("{game_version}-")) {
                    versions.push(value.clone());
                }
            }
            versions.sort();
            versions.dedup();
            Ok(versions)
        }
    }
}
