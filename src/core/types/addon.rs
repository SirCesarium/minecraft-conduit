use serde::{Deserialize, Serialize};

use crate::core::types::{
    loader::LoaderKind,
    source::{LockfileSource, ManifestSource},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ManifestAddon {
    Mod {
        loader: LoaderKind,
        source: ManifestSource,
    },
    Plugin {
        loader: LoaderKind,
        source: ManifestSource,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum LockfileAddon {
    Mod {
        loader: LoaderKind,
        source: LockfileSource,
        dependencies: Vec<Box<str>>,
    },
    Plugin {
        loader: LoaderKind,
        source: LockfileSource,
        dependencies: Vec<Box<str>>,
    },
}
