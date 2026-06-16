use std::io;

use reqwest::Error as ReqwestError;
use thiserror::Error;

use minecraft_registry_api::error::ApiError;

use crate::core::types::source::LockfileSource;

pub type ResolveResult = Result<LockfileSource, ResolverError>;

#[derive(Debug, Error)]
pub enum ResolverError {
    #[error("{0}")]
    Message(Box<str>),

    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Modrinth API error: {0}")]
    Modrinth(#[from] ApiError),

    #[error("Project not found: {slug}")]
    ProjectNotFound { slug: Box<str> },

    #[error("No matching version found for {0}")]
    NoMatchingVersion(Box<str>),

    #[error("No downloadable files for version {version} of {project}")]
    NoFiles {
        project: Box<str>,
        version: Box<str>,
    },
}

impl From<ReqwestError> for ResolverError {
    fn from(e: ReqwestError) -> Self {
        Self::Message(e.to_string().into())
    }
}
