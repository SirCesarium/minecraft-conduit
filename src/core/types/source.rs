use std::collections::BTreeMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
pub enum VersionConstraint {
    Exact(Box<str>),
    Latest,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum ManifestSource {
    Modrinth {
        slug: Option<Box<str>>,
        version: VersionConstraint,
    },
    #[serde(rename = "github")]
    GitHub {
        owner: Box<str>,
        repo: Box<str>,
        tag: VersionConstraint,
        file: Box<str>,
    },
    Url(Box<str>),
    Local(PathBuf),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hashes {
    pub algorithms: BTreeMap<Box<str>, Box<str>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum LockfileSource {
    Modrinth {
        project_id: Box<str>,
        version_id: Box<str>,
        filename: Box<str>,
        url: Box<str>,
        hashes: Hashes,
    },
    #[serde(rename = "github")]
    GitHub {
        owner: Box<str>,
        repo: Box<str>,
        tag: Box<str>,
        filename: Box<str>,
        url: Box<str>,
        hashes: Hashes,
    },
    Url {
        url: Box<str>,
        hashes: Hashes,
    },
    Local {
        filename: Box<str>,
        hashes: Hashes,
    },
}
