pub mod mods;
pub mod plugins;

use std::future::Future;
use std::path::{Path, PathBuf};

use crate::core::model::addon::AddonKind;
use crate::core::model::loader::LoaderKind;
use crate::core::model::source::{LockfileSource, ManifestSource};
use crate::errors::Result;

pub struct SearchResult {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub downloads: i64,
    pub icon_url: String,
}

pub struct SearchResults {
    pub hits: Vec<SearchResult>,
    pub total: u64,
}

pub struct SearchParams<'a> {
    pub query: &'a str,
    pub loader: Option<LoaderKind>,
    pub game_version: Option<&'a str>,
    pub loader_version: Option<&'a str>,
    pub limit: usize,
    pub offset: usize,
    pub http: &'a reqwest::Client,
}

pub trait DownloadProgress: Send {
    fn on_progress(&mut self, downloaded: u64, total: Option<u64>);
    fn on_complete(&mut self, _path: &Path) {}
}

pub trait AddonProvider: Send + Sync {
    fn supports(&self, source: &ManifestSource) -> bool;
    fn supports_search(&self) -> bool;

    fn search(
        &self,
        params: SearchParams<'_>,
    ) -> impl Future<Output = Result<SearchResults>> + Send;

    fn download(
        &self,
        source: &ManifestSource,
        kind: AddonKind,
        game_version: Option<&str>,
        dest: &Path,
        http: &reqwest::Client,
        progress: &mut dyn DownloadProgress,
    ) -> impl Future<Output = Result<(LockfileSource, PathBuf)>> + Send;
}

pub struct NullProgress;

impl DownloadProgress for NullProgress {
    fn on_progress(&mut self, _downloaded: u64, _total: Option<u64>) {}
}
