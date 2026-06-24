use std::fmt;

use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, EnumIter)]
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

impl fmt::Display for LoaderKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Vanilla => "Vanilla",
            Self::Fabric => "Fabric",
            Self::Forge => "Forge",
            Self::NeoForge => "NeoForge",
            Self::Paper => "Paper",
            Self::Purpur => "Purpur",
        };
        f.write_str(name)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Loader {
    pub kind: LoaderKind,
    pub game_version: Option<String>,
    #[serde(default)]
    pub build: Option<String>,
}

impl Default for Loader {
    fn default() -> Self {
        Self {
            kind: LoaderKind::Vanilla,
            game_version: Some(String::new()),
            build: None,
        }
    }
}
