use serde::{Deserialize, Serialize};

use crate::core::model::{
    loader::LoaderKind,
    source::{LockfileSource, ManifestSource},
};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Side {
    Client,
    Server,
    Both,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AddonKind {
    Mod,
    Plugin,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManifestAddon {
    pub id: String,
    pub kind: AddonKind,
    pub side: Side,
    pub loader: LoaderKind,
    pub source: ManifestSource,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LockfileAddon {
    pub id: String,
    pub kind: AddonKind,
    pub side: Side,
    pub loader: LoaderKind,
    pub source: LockfileSource,
    pub dependencies: Vec<String>,
}
