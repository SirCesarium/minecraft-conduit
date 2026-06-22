use std::path::Path;

use tokio::fs;

use crate::core::schema::constants::LOCKFILE_HEADER;
use crate::core::schema::lockfile::ConduitLockfile;
use crate::errors::{Error, Result};

impl ConduitLockfile {
    pub async fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();

        let content = fs::read_to_string(&path)
            .await
            .map_err(|source| Error::Read {
                path: path.clone(),
                source,
            })?;

        let content = content
            .lines()
            .filter(|line| !line.starts_with('#'))
            .collect::<Vec<_>>()
            .join("\n");

        let lockfile: Self =
            toml::from_str(&content).map_err(|source| Error::TomlDeserialize { path, source })?;

        Ok(lockfile)
    }

    pub async fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref().to_path_buf();
        let content = toml::to_string_pretty(self)?;
        let content = format!("{LOCKFILE_HEADER}\n\n{content}");

        fs::write(&path, &content)
            .await
            .map_err(|source| Error::Write { path, source })?;

        Ok(())
    }

    pub async fn create(path: impl AsRef<Path>) -> Result<Self> {
        let lockfile = Self::default();

        lockfile.save(path).await?;

        Ok(lockfile)
    }
}
