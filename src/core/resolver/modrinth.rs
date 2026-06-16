use std::collections::BTreeMap;

use reqwest::Client;

use minecraft_registry_api::modrinth::models::{ProjectRef, VersionListQuery};
use minecraft_registry_api::modrinth::ModrinthClient;

use crate::core::context::ConduitContext;
use crate::core::resolver::ProviderResolver;
use crate::core::types::loader::LoaderKind;
use crate::errors::{ResolveResult, ResolverError};
use crate::core::types::source::{Hashes, LockfileSource, ManifestSource, VersionConstraint};

#[must_use]
pub fn loader_to_modrinth(loader: LoaderKind) -> Option<&'static str> {
    match loader {
        LoaderKind::Fabric => Some("fabric"),
        LoaderKind::Forge => Some("forge"),
        LoaderKind::NeoForge => Some("neoforge"),
        LoaderKind::Paper => Some("paper"),
        LoaderKind::Purpur => Some("purpur"),
        LoaderKind::Vanilla => None,
    }
}

pub struct ModrinthResolver {
    client: ModrinthClient,
}

impl ModrinthResolver {
    #[must_use]
    pub fn new(client: Client) -> Self {
        Self {
            client: ModrinthClient::new(client),
        }
    }
}

impl ProviderResolver for ModrinthResolver {
    fn is_updatable(&self) -> bool {
        true
    }

    fn provider_name(&self) -> &str {
        "modrinth"
    }

    fn supports(&self, source: &ManifestSource) -> bool {
        matches!(source, ManifestSource::Modrinth { .. })
    }

    async fn resolve(
        &self,
        id: &str,
        source: &ManifestSource,
        ctx: &ConduitContext,
    ) -> ResolveResult {
        let (slug_opt, version) = match source {
            ManifestSource::Modrinth { slug, version } => (slug, version),
            _ => {
                return Err(ResolverError::Message(
                    "ModrinthResolver called with non-Modrinth source".into(),
                ))
            }
        };

        let slug: &str = slug_opt.as_deref().unwrap_or(id);

        let loader_kind = ctx.loader_kind().ok_or_else(|| {
            ResolverError::Message("ConduitContext has no manifest loaded".into())
        })?;

        let Some(loader) = loader_to_modrinth(loader_kind) else {
            return Err(ResolverError::Message(
                "Vanilla loader is not supported for Modrinth resolution".into(),
            ));
        };

        let Some(game_version) = ctx.game_version() else {
            return Err(ResolverError::Message(
                "No game version in manifest".into(),
            ));
        };

        let project = self
            .client
            .get_project(ProjectRef { slug })
            .await?;
        let project_id = &project.id;

        let versions = self
            .client
            .get_versions(VersionListQuery { project_id })
            .await?;

        let matching: Vec<_> = versions
            .into_iter()
            .filter(|v| {
                v.game_versions.iter().any(|gv| gv == game_version)
                    && v.loaders.iter().any(|l| l == loader)
            })
            .collect();

        let selected = match version {
            VersionConstraint::Latest => matching.into_iter().next(),
            VersionConstraint::Exact(target) => matching
                .into_iter()
                .find(|v| v.version_number == target.as_ref()),
        };

        let Some(ver) = selected else {
            return Err(ResolverError::NoMatchingVersion(slug.into()));
        };

        let file = ver
            .files
            .iter()
            .find(|f| f.primary)
            .or_else(|| ver.files.first())
            .ok_or_else(|| ResolverError::NoFiles {
                project: slug.into(),
                version: ver.version_number.clone().into(),
            })?;

        let mut algorithms = BTreeMap::new();
        algorithms.insert("sha1".into(), file.hashes.sha1.clone().into());
        algorithms.insert("sha512".into(), file.hashes.sha512.clone().into());

        Ok(LockfileSource::Modrinth {
            project_id: ver.project_id.clone().into(),
            version_id: ver.id.clone().into(),
            filename: file.filename.clone().into(),
            url: file.url.clone().into(),
            hashes: Hashes { algorithms },
        })
    }
}
