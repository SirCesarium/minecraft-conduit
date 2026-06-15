use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LoaderKind {
    Vanilla,
    Fabric,
    Forge,
    #[serde(rename = "neoforge")]
    NeoForge,
    Paper,
    Purpur,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Loader {
    Vanilla {
        game_version: Box<str>,
    },
    Fabric {
        version: Box<str>,
        game_version: Box<str>,
    },
    Forge {
        version: Box<str>,
        game_version: Box<str>,
    },
    #[serde(rename = "neoforge")]
    NeoForge {
        version: Box<str>,
    },
    Paper {
        game_version: Box<str>,
        build: Box<str>,
    },
    Purpur {
        game_version: Box<str>,
        build: Box<str>,
    },
}

impl Loader {
    pub fn kind(&self) -> LoaderKind {
        match self {
            Self::Vanilla { .. } => LoaderKind::Vanilla,
            Self::Fabric { .. } => LoaderKind::Fabric,
            Self::Forge { .. } => LoaderKind::Forge,
            Self::NeoForge { .. } => LoaderKind::NeoForge,
            Self::Paper { .. } => LoaderKind::Paper,
            Self::Purpur { .. } => LoaderKind::Purpur,
        }
    }
}
