use std::future::Future;

use crate::core::types::loader::LoaderKind;
use crate::core::types::source::ManifestSource;
use crate::errors::ResolveResult;

pub mod github;
pub mod local;
pub mod modrinth;
pub mod url;

pub struct ResolveContext {
    pub game_version: Box<str>,
    pub loader: LoaderKind,
}

pub trait ProviderResolver {
    fn is_updatable(&self) -> bool;
    fn provider_name(&self) -> &str;
    fn supports(&self, source: &ManifestSource) -> bool;
    fn resolve(
        &self,
        id: &str,
        source: &ManifestSource,
        ctx: &ResolveContext,
    ) -> impl Future<Output = ResolveResult>;
}
