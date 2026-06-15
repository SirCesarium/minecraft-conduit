use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::core::types::{addon::LockfileAddon, loader::Loader};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConduitLockfile {
    pub lock_version: u32,
    pub java: Option<Box<str>>,
    pub loader: Loader,
    pub dependencies: BTreeMap<Box<str>, LockfileAddon>,
}

impl Default for ConduitLockfile {
    fn default() -> Self {
        Self {
            lock_version: 1,
            java: None,
            loader: Loader::default(),
            dependencies: BTreeMap::new(),
        }
    }
}

impl ConduitLockfile {
    #[must_use]
    pub fn new(lock_version: u32, loader: Loader, java: Option<Box<str>>) -> Self {
        Self {
            lock_version,
            java,
            loader,
            dependencies: BTreeMap::new(),
        }
    }
}
