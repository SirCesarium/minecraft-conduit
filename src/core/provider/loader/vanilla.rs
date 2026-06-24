use minecraft_registry_api::error::ApiError;
use minecraft_registry_api::mojang::MojangClient;

use super::VersionProvider;

use crate::core::model::addon::AddonKind;
use crate::core::provider::loader::AddonFolderProvider;

pub struct VanillaProvider;

impl VersionProvider for VanillaProvider {
    async fn fetch_game_versions(&self, http: &reqwest::Client) -> Result<Vec<String>, ApiError> {
        let mojang = MojangClient::new(http.clone());
        let manifest = mojang.get_manifest().await?;
        Ok(manifest
            .versions
            .into_iter()
            .filter(|v| v.type_field == "release")
            .map(|v| v.id)
            .collect())
    }

    async fn fetch_loader_versions(
        &self,
        _http: &reqwest::Client,
        _game_version: &str,
    ) -> Result<Vec<String>, ApiError> {
        Ok(vec![])
    }
}

impl AddonFolderProvider for VanillaProvider {
    fn get_addon_folder(&self, _: AddonKind) -> Option<&'static str> {
        None
    }
}
