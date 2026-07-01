use std::path::{Path, PathBuf};

use crate::core::model::addon::AddonKind;
use crate::core::model::source::{LockfileSource, ManifestSource};
use crate::errors::Result;

use super::super::{AddonProvider, DownloadProgress, SearchParams, SearchResults};

pub struct GithubModProvider;

impl AddonProvider for GithubModProvider {
    fn supports(&self, _source: &ManifestSource) -> bool {
        false
    }

    fn supports_search(&self) -> bool {
        todo!()
    }

    async fn search(
        &self,
        _params: SearchParams<'_>,
    ) -> Result<SearchResults> {
        todo!()
    }

    async fn download(
        &self,
        _source: &ManifestSource,
        _kind: AddonKind,
        _game_version: Option<&str>,
        _dest: &Path,
        _http: &reqwest::Client,
        _progress: &mut dyn DownloadProgress,
    ) -> Result<(LockfileSource, PathBuf)> {
        todo!()
    }
}
