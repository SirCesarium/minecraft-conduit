use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use minecraft_registry_api::modrinth::models::{
    Facet, ProjectRef, ProjectType, SearchQuery, VersionListQuery,
};
use minecraft_registry_api::modrinth::ModrinthClient;

use crate::core::model::addon::AddonKind;
use crate::core::model::loader::LoaderKind;
use crate::core::model::source::{Hashes, LockfileSource, ManifestSource, VersionConstraint};
use crate::errors::{Error, Result};

use super::super::{AddonProvider, DownloadProgress, SearchParams, SearchResult, SearchResults};

pub struct ModrinthModProvider;

impl AddonProvider for ModrinthModProvider {
    fn supports(&self, source: &ManifestSource) -> bool {
        matches!(source, ManifestSource::Modrinth { .. })
    }

    fn supports_search(&self) -> bool {
        true
    }

    async fn search(
        &self,
        params: SearchParams<'_>,
    ) -> Result<SearchResults> {
        let client = ModrinthClient::new(params.http.clone());

        let mut facets: Vec<Vec<Facet>> =
            vec![vec![Facet::ProjectType(ProjectType::Mod)]];

        if let Some(loader) = params.loader {
            let loader_name = match loader {
                LoaderKind::Fabric => "fabric",
                LoaderKind::Forge => "forge",
                LoaderKind::NeoForge => "neoforge",
                LoaderKind::Paper => "paper",
                LoaderKind::Purpur => "purpur",
                LoaderKind::Vanilla => "vanilla",
            };
            facets.push(vec![Facet::Loader(loader_name.to_string())]);
        }

        if let Some(version) = params.game_version {
            facets.push(vec![Facet::GameVersion(version.to_string())]);
        }

        let result = client
            .search(SearchQuery {
                query: params.query,
                limit: params.limit as u32,
                offset: params.offset as u32,
                facets: Some(facets),
            })
            .await?;

        let total = result.total_hits as u64;

        let hits: Vec<SearchResult> = result
            .hits
            .into_iter()
            .map(|hit| SearchResult {
                slug: hit.slug,
                title: hit.title,
                description: hit.description,
                downloads: hit.downloads,
                icon_url: hit.icon_url,
            })
            .collect();

        Ok(SearchResults { hits, total })
    }

    async fn download(
        &self,
        source: &ManifestSource,
        _kind: AddonKind,
        game_version: Option<&str>,
        dest: &Path,
        http: &reqwest::Client,
        progress: &mut dyn DownloadProgress,
    ) -> Result<(LockfileSource, PathBuf)> {
        let (slug, version_constraint) = match source {
            ManifestSource::Modrinth { slug, version } => (slug.as_str(), version),
            _ => return Err(Error::AddonNotFound("not a modrinth source".to_string())),
        };

        let client = ModrinthClient::new(http.clone());

        let project = client
            .get_project(ProjectRef { slug })
            .await
            .map_err(|e| Error::AddonNotFound(format!("{slug}: {e}")))?;

        let versions = client
            .get_versions(VersionListQuery {
                project_id: &project.id,
            })
            .await?;

        let mut filtered: Vec<_> = versions
            .into_iter()
            .filter(|v| {
                if let Some(gv) = game_version {
                    v.game_versions.iter().any(|gv_| gv_ == gv)
                } else {
                    true
                }
            })
            .collect();

        filtered.sort_by(|a, b| b.date_published.cmp(&a.date_published));

        let version = match version_constraint {
            VersionConstraint::Exact(req) => filtered
                .into_iter()
                .find(|v| v.version_number == *req)
                .ok_or_else(|| {
                    Error::AddonNotFound(format!("{slug}: version {req} not found"))
                })?,
            VersionConstraint::Latest => filtered
                .into_iter()
                .next()
                .ok_or_else(|| Error::AddonNotFound(format!("{slug}: no versions found")))?,
        };

        let version_id = version.id.clone();
        let file_idx = version.files.iter().position(|f| f.primary).unwrap_or(0);
        let file = version.files.into_iter().nth(file_idx).ok_or_else(|| {
            Error::AddonNotFound(format!("{slug}: no files in version {version_id}"))
        })?;

        let total = file.size as u64;
        progress.on_progress(0, Some(total));

        let bytes = http
            .get(&file.url)
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        progress.on_progress(bytes.len() as u64, Some(total));

        use tokio::fs;

        fs::create_dir_all(dest).await?;
        let path = dest.join(&file.filename);
        fs::write(&path, &bytes).await?;
        progress.on_complete(&path);

        let mut algorithms = BTreeMap::new();
        algorithms.insert("sha1".to_string(), file.hashes.sha1);
        algorithms.insert("sha512".to_string(), file.hashes.sha512);

        let lockfile_source = LockfileSource::Modrinth {
            project_id: project.id,
            version_id,
            filename: file.filename,
            url: file.url,
            hashes: Hashes { algorithms },
        };

        Ok((lockfile_source, path))
    }
}
