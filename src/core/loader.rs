use crate::core::types::loader::{Loader, LoaderKind};

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
