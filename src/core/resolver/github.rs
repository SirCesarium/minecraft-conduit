use std::collections::BTreeMap;

use reqwest::Client;
use serde_json::Value;

use crate::core::context::ConduitContext;
use crate::core::resolver::ProviderResolver;
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

    async fn fetch_release(&self, url: &str) -> Result<Value, ResolverError> {
        let resp: Value = self
            .client
            .get(url)
            .header("User-Agent", "conduit")
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(resp)
    }

    fn parse_digest(asset: &Value) -> Result<BTreeMap<Box<str>, Box<str>>, ResolverError> {
        let digest = asset
            .get("digest")
            .and_then(|d| d.as_str())
            .ok_or_else(|| {
                ResolverError::Message("GitHub: asset has no digest".into())
            })?;

        let parts: Vec<&str> = digest.splitn(2, ':').collect();
        let (algo, hash) = (
            *parts.first().ok_or_else(|| {
                ResolverError::Message("GitHub: malformed digest".into())
            })?,
            *parts.get(1).ok_or_else(|| {
                ResolverError::Message("GitHub: malformed digest".into())
            })?,
        );

        let mut algorithms = BTreeMap::new();
        algorithms.insert(algo.into(), hash.into());
        Ok(algorithms)
    }

    fn resolve_from_release(
        release: &Value,
        owner: &str,
        repo: &str,
        file: &str,
    ) -> ResolveResult {
        let tag = release
            .get("tag_name")
            .and_then(|t| t.as_str())
            .ok_or_else(|| {
                ResolverError::Message("GitHub: release has no tag_name".into())
            })?;

        let assets = release
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
            .get("browser_download_url")
            .and_then(|u| u.as_str())
            .ok_or_else(|| {
                ResolverError::Message(
                    "GitHub: asset has no download URL".into(),
                )
            })?;

        let algorithms = Self::parse_digest(asset)?;

        Ok(LockfileSource::GitHub {
            owner: owner.into(),
            repo: repo.into(),
            tag: tag.into(),
            filename: file.into(),
            url: download_url.into(),
            hashes: Hashes { algorithms },
        })
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
        _ctx: &ConduitContext,
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
                let release = self.fetch_release(&url).await?;
                return Self::resolve_from_release(&release, owner, repo, file);
            }
        };

        let url = format!(
            "https://api.github.com/repos/{owner}/{repo}/releases/tags/{tag_str}"
        );
        let release = self.fetch_release(&url).await?;
        Self::resolve_from_release(&release, owner, repo, file)
    }
}
