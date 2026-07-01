use std::path::{Path, PathBuf};

use crate::core::model::addon::AddonKind;
use crate::core::model::source::{LockfileSource, ManifestSource};
use crate::errors::{Error, Result};

use crate::core::provider::addon::{AddonProvider, DownloadProgress, SearchParams, SearchResults};

pub struct LocalPluginProvider;

impl AddonProvider for LocalPluginProvider {
    fn supports(&self, _source: &ManifestSource) -> bool {
        false
    }

    fn supports_search(&self) -> bool {
        false
    }

    async fn search(
        &self,
        _params: SearchParams<'_>,
    ) -> Result<SearchResults> {
        Err(Error::SearchNotSupported)
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
