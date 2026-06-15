use std::collections::BTreeMap;

use reqwest::Client;
use sha2::{Digest, Sha256};

use crate::core::resolver::{ProviderResolver, ResolveContext};
use crate::errors::{ResolveResult, ResolverError};
use crate::core::types::source::{Hashes, LockfileSource, ManifestSource};

pub struct UrlResolver {
    client: Client,
}

impl UrlResolver {
    #[must_use]
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

impl ProviderResolver for UrlResolver {
    fn is_updatable(&self) -> bool {
        true
    }

    fn provider_name(&self) -> &str {
        "url"
    }

    fn supports(&self, source: &ManifestSource) -> bool {
        matches!(source, ManifestSource::Url(..))
    }

    async fn resolve(
        &self,
        _id: &str,
        source: &ManifestSource,
        _ctx: &ResolveContext,
    ) -> ResolveResult {
        let url_str = match source {
            ManifestSource::Url(u) => u.as_ref(),
            _ => {
                return Err(ResolverError::Message(
                    "UrlResolver called with non-URL source".into(),
                ))
            }
        };

        let data = self
            .client
            .get(url_str)
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        let hash = Sha256::digest(&data);
        let sha256: String = hash.iter().map(|b| format!("{b:02x}")).collect();

        let mut algorithms = BTreeMap::new();
        algorithms.insert("sha256".into(), sha256.into());

        Ok(LockfileSource::Url {
            url: (*url_str).into(),
            hashes: Hashes { algorithms },
        })
    }
}
