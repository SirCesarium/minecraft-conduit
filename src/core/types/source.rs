use std::collections::BTreeMap;
use std::path::PathBuf;

pub enum VersionConstraint {
    Exact(Box<str>),
    Latest,
    Range(Box<str>),
}

pub enum ManifestSource {
    Modrinth {
        slug: Option<Box<str>>,
        version: VersionConstraint,
    },
    GitHub {
        owner: Box<str>,
        repo: Box<str>,
        tag: VersionConstraint,
        file: Box<str>,
    },
    Url(Box<str>),
    Local(PathBuf),
}

pub struct Hashes {
    pub algorithms: BTreeMap<Box<str>, Box<str>>,
}

pub enum LockfileSource {
    Modrinth {
        project_id: Box<str>,
        version_id: Box<str>,
        filename: Box<str>,
        url: Box<str>,
        hashes: Hashes,
    },
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
