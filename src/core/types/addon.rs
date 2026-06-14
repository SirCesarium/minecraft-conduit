use crate::core::types::{
    loader::LoaderKind,
    source::{LockfileSource, ManifestSource},
};

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

pub enum LockfileAddon {
    Mod {
        id: Box<str>,
        loader: LoaderKind,
        source: LockfileSource,
        dependencies: Vec<Box<str>>,
    },
    Plugin {
        id: Box<str>,
        loader: LoaderKind,
        source: LockfileSource,
        dependencies: Vec<Box<str>>,
    },
}
