use crate::core::types::{addon::ManifestAddon, loader::Loader};

pub struct ConduitManifest {
    pub name: String,
    pub loader: Loader,
    pub dependencies: Vec<ManifestAddon>,
}
