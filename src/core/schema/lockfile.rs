use std::collections::BTreeMap;

use crate::core::types::{addon::LockfileAddon, loader::Loader};

pub struct ConduitLockfile {
    pub lock_version: u32,
    pub loader: Loader,
    pub dependencies: BTreeMap<Box<str>, LockfileAddon>,
}
