use std::future::Future;

use minecraft_registry_api::error::ApiError;
use minecraft_registry_api::fabric::FabricClient;

use crate::core::model::addon::AddonKind;
use crate::core::provider::loader::AddonFolderProvider;

use super::VersionProvider;

pub struct FabricProvider;

impl VersionProvider for FabricProvider {
    fn fetch_game_versions(&self, http: &reqwest::Client) -> impl Future<Output = Result<Vec<String>, ApiError>> + Send {
        let http = http.clone();
        async move {
            let fabric = FabricClient::new(http);
            let versions = fabric.get_game_versions().await?;
            Ok(versions
                .into_iter()
                .filter(|v| v.stable)
                .map(|v| v.version)
                .collect())
        }
    }

    fn fetch_loader_versions(
        &self,
        http: &reqwest::Client,
        _game_version: &str,
    ) -> impl Future<Output = Result<Vec<String>, ApiError>> + Send {
        let http = http.clone();
        async move {
            let fabric = FabricClient::new(http);
            let versions = fabric.get_loaders().await?;
            Ok(versions
                .into_iter()
                .map(|v| v.version)
                .collect())
        }
    }
}

impl AddonFolderProvider for FabricProvider {
    fn get_addon_folder(&self, r#type: AddonKind) -> Option<&'static str> {
        match r#type {
            AddonKind::Mod => Some("mods"),
            AddonKind::Plugin => None,
        }
    }
}
