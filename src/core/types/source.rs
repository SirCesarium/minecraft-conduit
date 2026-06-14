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

pub enum LockfileSource {
    Modrinth { id: Box<str>, version: Box<str> },
    Local(PathBuf),
}
