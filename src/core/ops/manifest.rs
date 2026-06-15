use std::fs;
use std::io;
use std::path::Path;

use crate::core::ops::{ManifestManager, ManifestOps};
use crate::core::schema::manifest::ConduitManifest;

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
