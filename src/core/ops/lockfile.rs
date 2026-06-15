use std::fs;
use std::io;
use std::path::Path;

use crate::core::ops::{LockfileManager, LockfileOps};
use crate::core::schema::lockfile::ConduitLockfile;

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
