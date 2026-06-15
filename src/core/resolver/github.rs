use std::collections::BTreeMap;

use reqwest::Client;
use sha2::{Digest, Sha256};

use crate::core::resolver::{ProviderResolver, ResolveContext};
use crate::errors::{ResolveResult, ResolverError};
use crate::core::types::source::{Hashes, LockfileSource, ManifestSource, VersionConstraint};

pub struct GitHubResolver {
    client: Client,
}

impl GitHubResolver {
    #[must_use]
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

impl ProviderResolver for GitHubResolver {
    fn is_updatable(&self) -> bool {
        true
    }

    fn provider_name(&self) -> &str {
        "github"
    }

    fn supports(&self, source: &ManifestSource) -> bool {
        matches!(source, ManifestSource::GitHub { .. })
    }

    async fn resolve(
        &self,
        _id: &str,
        source: &ManifestSource,
        _ctx: &ResolveContext,
    ) -> ResolveResult {
        let (owner, repo, tag, file) = match source {
            ManifestSource::GitHub {
                owner,
                repo,
                tag,
                file,
            } => (owner, repo, tag, file),
            _ => {
                return Err(ResolverError::Message(
                    "GitHubResolver called with non-GitHub source".into(),
                ))
            }
        };

        let tag_str = match tag {
            VersionConstraint::Exact(t) => t.as_ref(),
            VersionConstraint::Latest => {
                let url = format!(
                    "https://api.github.com/repos/{owner}/{repo}/releases/latest"
                );
                let resp: serde_json::Value = self
                    .client
                    .get(&url)
                    .header("User-Agent", "conduit")
                    .send()
                    .await?
                    .error_for_status()?
                    .json()
                    .await?;
                let t = resp
                    .get("tag_name")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        ResolverError::Message(
                            "GitHub: could not determine latest tag".into(),
                        )
                    })?;
                // Can't return reference to local, download directly
                return self.download_asset(owner, repo, t, file).await;
            }
        };

        self.download_asset(owner, repo, tag_str, file).await
    }
}

impl GitHubResolver {
    fn compute_sha256(data: &[u8]) -> Box<str> {
        let hash = Sha256::digest(data);
        hash.iter().map(|b| format!("{b:02x}")).collect::<String>().into()
    }

    async fn download_asset(
        &self,
        owner: &str,
        repo: &str,
        tag: &str,
        file: &str,
    ) -> ResolveResult {
        let api_url = format!(
            "https://api.github.com/repos/{owner}/{repo}/releases/tags/{tag}"
        );

        let resp: serde_json::Value = self
            .client
            .get(&api_url)
            .header("User-Agent", "conduit")
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        let assets = resp
            .get("assets")
            .and_then(|a| a.as_array())
            .ok_or_else(|| {
                ResolverError::Message("GitHub: no assets in release".into())
            })?;

        let asset = assets
            .iter()
            .find(|a| a.get("name").and_then(|n| n.as_str()) == Some(file))
            .ok_or_else(|| {
                ResolverError::Message(
                    format!("GitHub: asset '{file}' not found in release {tag}").into(),
                )
            })?;

        let download_url = asset
            .get("url")
            .and_then(|u| u.as_str())
            .ok_or_else(|| {
                ResolverError::Message(
                    "GitHub: asset has no download URL".into(),
                )
            })?;

        let data = self
            .client
            .get(download_url)
            .header("User-Agent", "conduit")
            .header("Accept", "application/octet-stream")
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        let sha256 = Self::compute_sha256(&data);
        let mut algorithms = BTreeMap::new();
        algorithms.insert("sha256".into(), sha256);

        Ok(LockfileSource::GitHub {
            owner: (*owner).into(),
            repo: (*repo).into(),
            tag: (*tag).into(),
            filename: (*file).into(),
            url: download_url.into(),
            hashes: Hashes { algorithms },
        })
    }
}
