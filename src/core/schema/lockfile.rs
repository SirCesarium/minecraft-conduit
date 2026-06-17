use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::model::{addon::LockfileAddon, loader::Loader};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConduitLockfile {
    pub lock_version: u32,
    pub loader: Loader,
    pub dependencies: BTreeMap<Uuid, LockfileAddon>,
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

    pub fn add_dependency(&mut self, dep: LockfileAddon) -> Uuid {
        let id = Uuid::new_v4();
        self.dependencies.insert(id, dep);
        id
    }

    pub fn remove_dependency(&mut self, id: &Uuid) -> Option<LockfileAddon> {
        self.dependencies.remove(id)
    }

    #[must_use]
    pub fn has_dependency(&self, id: &Uuid) -> bool {
        self.dependencies.contains_key(id)
    }
}
