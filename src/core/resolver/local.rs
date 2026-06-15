use std::collections::BTreeMap;
use std::path::Path;

use sha2::{Digest, Sha256};
use tokio::fs;

use crate::core::resolver::{ProviderResolver, ResolveContext};
use crate::errors::{ResolveResult, ResolverError};
use crate::core::types::source::{Hashes, LockfileSource, ManifestSource};

pub struct LocalResolver;

impl LocalResolver {
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl Default for LocalResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl ProviderResolver for LocalResolver {
    fn is_updatable(&self) -> bool {
        false
    }

    fn provider_name(&self) -> &str {
        "local"
    }

    fn supports(&self, source: &ManifestSource) -> bool {
        matches!(source, ManifestSource::Local(..))
    }

    async fn resolve(
        &self,
        _id: &str,
        source: &ManifestSource,
        _ctx: &ResolveContext,
    ) -> ResolveResult {
        let path = match source {
            ManifestSource::Local(p) => p,
            _ => {
                return Err(ResolverError::Message(
                    "LocalResolver called with non-Local source".into(),
                ))
            }
        };

        let data = fs::read(path).await?;

        let hash = Sha256::digest(&data);
        let sha256: String = hash.iter().map(|b| format!("{b:02x}")).collect();

        let filename = Path::new(path)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path.to_string_lossy().to_string());

        let mut algorithms = BTreeMap::new();
        algorithms.insert("sha256".into(), sha256.into());

        Ok(LockfileSource::Local {
            filename: filename.into(),
            hashes: Hashes { algorithms },
        })
    }
}
