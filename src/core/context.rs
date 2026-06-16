use reqwest::Client;

use crate::core::schema::lockfile::ConduitLockfile;
use crate::core::schema::manifest::ConduitManifest;
use crate::core::types::loader::{Loader, LoaderKind};

pub struct ConduitContext {
    pub client: Client,
    pub manifest: Option<ConduitManifest>,
    pub lockfile: Option<ConduitLockfile>,
}

impl ConduitContext {
    #[must_use]
    pub fn new(client: Client) -> Self {
        Self {
            client,
            manifest: None,
            lockfile: None,
        }
    }

    #[must_use]
    pub fn game_version(&self) -> Option<&str> {
        let m = self.manifest.as_ref()?;
        match &m.loader {
            Loader::Vanilla { game_version }
            | Loader::Fabric { game_version, .. }
            | Loader::Forge { game_version, .. }
            | Loader::Paper { game_version, .. }
            | Loader::Purpur { game_version, .. } => Some(game_version.as_ref()),
            Loader::NeoForge { .. } => None,
        }
    }

    #[must_use]
    pub fn loader_kind(&self) -> Option<LoaderKind> {
        Some(self.manifest.as_ref()?.loader.kind())
    }
}
