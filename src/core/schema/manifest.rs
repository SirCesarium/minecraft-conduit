use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::core::types::{addon::ManifestAddon, loader::Loader};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConduitManifest {
    pub name: Box<str>,
    pub java: Option<Box<str>>,
    pub loader: Loader,
    pub dependencies: BTreeMap<Box<str>, ManifestAddon>,
}
