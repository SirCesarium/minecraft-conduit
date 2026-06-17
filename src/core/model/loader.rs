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
pub struct Loader {
    pub kind: LoaderKind,
    pub game_version: Option<String>,
    pub version: Option<String>,
    pub build: Option<String>,
}

impl Default for Loader {
    fn default() -> Self {
        Self {
            kind: LoaderKind::Vanilla,
            game_version: Some(String::new()),
            version: None,
            build: None,
        }
    }
}
