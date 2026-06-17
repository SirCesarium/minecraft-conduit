use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::core::model::{addon::LockfileAddon, loader::Loader};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConduitLockfile {
    pub lock_version: u32,
    pub loader: Loader,
    pub dependencies: BTreeMap<String, LockfileAddon>,
}

impl Default for ConduitLockfile {
    fn default() -> Self {
        Self {
            lock_version: 1,
            loader: Loader::default(),
            dependencies: BTreeMap::new(),
        }
    }
}

impl ConduitLockfile {
    #[must_use]
    pub fn new(lock_version: u32, loader: Loader) -> Self {
        Self {
            lock_version,
            loader,
            dependencies: BTreeMap::new(),
        }
    }
}
