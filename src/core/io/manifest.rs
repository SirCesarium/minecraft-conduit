use std::path::Path;

use tokio::fs;

use crate::core::schema::manifest::ConduitManifest;
use crate::errors::{Error, Result};

impl ConduitManifest {
    pub async fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();

        let content = fs::read_to_string(&path)
            .await
            .map_err(|source| Error::Read {
                path: path.clone(),
                source,
            })?;

        let manifest: Self =
            toml::from_str(&content).map_err(|source| Error::TomlDeserialize { path, source })?;

        Ok(manifest)
    }

    pub async fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref().to_path_buf();

        let content = toml::to_string_pretty(self)?;

        fs::write(&path, &content)
            .await
            .map_err(|source| Error::Write { path, source })?;

        Ok(())
    }

    pub async fn create(path: impl AsRef<Path>, name: &str) -> Result<Self> {
        let manifest = Self::new(name, Default::default());

        manifest.save(path).await?;

        Ok(manifest)
    }
}
