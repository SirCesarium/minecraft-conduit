use std::fs;
use std::io;
use std::path::Path;

use crate::core::schema::lockfile::ConduitLockfile;
use crate::core::schema::manifest::ConduitManifest;

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

impl ManifestOps for ManifestManager {
    type Error = io::Error;

    fn load(path: &Path) -> Result<ConduitManifest, Self::Error> {
        let data = fs::read_to_string(path)?;
        let manifest: ConduitManifest =
            toml::from_str(&data).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(manifest)
    }

    fn save(path: &Path, manifest: &ConduitManifest) -> Result<(), Self::Error> {
        let data =
            toml::to_string(manifest).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(path, data)
    }
}

impl LockfileOps for LockfileManager {
    type Error = io::Error;

    fn load(path: &Path) -> Result<ConduitLockfile, Self::Error> {
        let data = fs::read_to_string(path)?;
        let lockfile: ConduitLockfile =
            toml::from_str(&data).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(lockfile)
    }

    fn save(path: &Path, lockfile: &ConduitLockfile) -> Result<(), Self::Error> {
        let data =
            toml::to_string(lockfile).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(path, data)
    }
}
