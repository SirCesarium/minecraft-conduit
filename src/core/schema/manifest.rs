use serde::{Deserialize, Serialize};

use crate::core::model::{addon::ManifestAddon, loader::Loader};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConduitManifest {
    pub name: String,
    pub loader: Loader,
    pub dependencies: Vec<ManifestAddon>,
}

impl Default for ConduitManifest {
    fn default() -> Self {
        Self {
            name: "my-server".to_string(),
            loader: Loader::default(),
            dependencies: Vec::new(),
        }
    }
}

impl ConduitManifest {
    #[must_use]
    pub fn new(name: &str, loader: Loader) -> Self {
        Self {
            name: name.into(),
            loader,
            dependencies: Vec::new(),
        }
    }
}
