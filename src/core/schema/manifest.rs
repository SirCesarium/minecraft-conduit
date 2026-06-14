use std::collections::BTreeMap;

use crate::core::types::{addon::ManifestAddon, loader::Loader};

pub struct ConduitManifest {
    pub name: Box<str>,
    pub loader: Loader,
    pub dependencies: BTreeMap<Box<str>, ManifestAddon>,
}
