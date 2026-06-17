#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoaderKind {
    Vanilla,
    Fabric,
    Forge,
    NeoForge,
    Paper,
    Purpur,
}

#[derive(Debug, Clone, PartialEq)]
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
