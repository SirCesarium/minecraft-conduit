use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum VersionConstraint {
    Exact(String),
    Latest,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ManifestSource {
    Modrinth {
        slug: String,
        version: VersionConstraint,
    },
    GitHub {
        owner: String,
        repo: String,
        tag: VersionConstraint,
        file: String,
    },
    Url {
        url: String,
    },
    Local {
        path: PathBuf,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Hashes {
    pub algorithms: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LockfileSource {
    Modrinth {
        project_id: String,
        version_id: String,
        filename: String,
        url: String,
        hashes: Hashes,
    },
    GitHub {
        owner: String,
        repo: String,
        tag: String,
        filename: String,
        url: String,
        hashes: Hashes,
    },
    Url {
        url: String,
        hashes: Hashes,
    },
    Local {
        filename: String,
        hashes: Hashes,
    },
}
