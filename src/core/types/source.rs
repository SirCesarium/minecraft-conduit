use std::collections::BTreeMap;
use std::path::PathBuf;

pub enum VersionConstraint {
    Exact(Box<str>),
    Latest,
    Range(Box<str>),
}

pub enum ManifestSource {
    Modrinth {
        slug: Box<str>,
        version: VersionConstraint,
    },
    Local(PathBuf),
}

pub struct Hashes {
    pub algorithms: BTreeMap<Box<str>, Box<str>>,
}

pub enum LockfileSource {
    Modrinth {
        id: Box<str>,
        version: Box<str>,
        hashes: Hashes,
    },
    Local(PathBuf),
}
