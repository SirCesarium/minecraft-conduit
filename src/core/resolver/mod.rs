use std::future::Future;

use crate::core::context::ConduitContext;
use crate::core::types::source::ManifestSource;
use crate::errors::ResolveResult;

pub mod github;
pub mod local;
pub mod modrinth;
pub mod url;

pub trait ProviderResolver {
    fn is_updatable(&self) -> bool;
    fn provider_name(&self) -> &str;
    fn supports(&self, source: &ManifestSource) -> bool;
    fn resolve(
        &self,
        id: &str,
        source: &ManifestSource,
        ctx: &ConduitContext,
    ) -> impl Future<Output = ResolveResult>;
}
