use serde::{Deserialize, Serialize};

use crate::core::model::{addon::ManifestAddon, loader::Loader};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConduitManifest {
    pub name: String,
    pub loader: Loader,
    #[serde(skip_serializing_if = "Vec::is_empty")]
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

    pub fn add_dependency(&mut self, dep: ManifestAddon) {
        self.dependencies.push(dep);
    }

    pub fn remove_dependency(&mut self, id: &str) -> Option<ManifestAddon> {
        let pos = self.dependencies.iter().position(|d| d.id == id)?;
        Some(self.dependencies.remove(pos))
    }

    #[must_use]
    pub fn has_dependency(&self, id: &str) -> bool {
        self.dependencies.iter().any(|d| d.id == id)
    }
}
