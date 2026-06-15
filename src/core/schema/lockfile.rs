use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::core::types::{addon::LockfileAddon, loader::Loader};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConduitLockfile {
    pub lock_version: u32,
    pub java_min_version: Option<Box<str>>,
    pub loader: Loader,
    pub dependencies: BTreeMap<Box<str>, LockfileAddon>,
}
