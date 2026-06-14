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
        loader: LoaderKind,
        source: LockfileSource,
    },
    Plugin {
        loader: LoaderKind,
        source: LockfileSource,
    },
}
