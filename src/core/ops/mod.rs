use std::path::Path;

use crate::core::schema::lockfile::ConduitLockfile;
use crate::core::schema::manifest::ConduitManifest;

pub mod lockfile;
pub mod manifest;

pub struct ManifestManager;

pub struct LockfileManager;

pub trait ManifestOps {
    type Error;

    fn load(path: &Path) -> Result<ConduitManifest, Self::Error>;

    fn save(path: &Path, manifest: &ConduitManifest) -> Result<(), Self::Error>;
}

pub trait LockfileOps {
    type Error;

    fn load(path: &Path) -> Result<ConduitLockfile, Self::Error>;

    fn save(path: &Path, lockfile: &ConduitLockfile) -> Result<(), Self::Error>;
}
