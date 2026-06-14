use std::error::Error;
use std::fmt;
use std::future::Future;

use crate::core::types::source::{LockfileSource, ManifestSource};

pub trait ProviderResolver {
    fn is_updatable(&self) -> bool;
    fn provider_name(&self) -> &str;
    fn supports(&self, source: &ManifestSource) -> bool;
    fn resolve(
        &self,
        id: &str,
        source: &ManifestSource,
    ) -> impl Future<Output = Result<LockfileSource, ResolverError>>;
}

#[derive(Debug)]
pub struct ResolverError {
    pub message: Box<str>,
}

impl fmt::Display for ResolverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ResolverError {}
