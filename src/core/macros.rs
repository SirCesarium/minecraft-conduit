/// `version!(exact "1.21.4")`, `version!(latest)`, `version!(range ">=1.21")`
#[macro_export]
macro_rules! version {
    (exact $v:expr) => {
        $crate::core::types::source::VersionConstraint::Exact($v.into())
    };
    (latest) => {
        $crate::core::types::source::VersionConstraint::Latest
    };
    (range $v:expr) => {
        $crate::core::types::source::VersionConstraint::Range($v.into())
    };
}

/// `hashes! { sha1 = "...", sha512 = "..." }`
#[macro_export]
macro_rules! hashes {
    ($($algo:ident = $value:expr),+ $(,)?) => {{
        let mut map = ::std::collections::BTreeMap::new();
        $( map.insert(stringify!($algo).into(), $value.into()); )+
        $crate::core::types::source::Hashes { algorithms: map }
    }};
}

/// `modrinth!(slug = "sodium", version = version!(exact "1.0.0"))`
#[macro_export]
macro_rules! modrinth {
    (slug = $slug:expr, version = $version:expr) => {
        $crate::core::types::source::ManifestSource::Modrinth {
            slug: Some($slug.into()),
            version: $version,
        }
    };
    (version = $version:expr $(,)?) => {
        $crate::core::types::source::ManifestSource::Modrinth {
            slug: None,
            version: $version,
        }
    };
}

/// `github!(owner = "CaffeineMC", repo = "sodium", tag = version!(exact "mc1.21.4-0.6.5"), file = "sodium.jar")`
#[macro_export]
macro_rules! github {
    (owner = $owner:expr, repo = $repo:expr, tag = $tag:expr, file = $file:expr) => {
        $crate::core::types::source::ManifestSource::GitHub {
            owner: $owner.into(),
            repo: $repo.into(),
            tag: $tag,
            file: $file.into(),
        }
    };
}

/// `url!("https://example.com/mod.jar")`
#[macro_export]
macro_rules! url {
    ($url:expr) => {
        $crate::core::types::source::ManifestSource::Url($url.into())
    };
}

/// `local!("mods/sodium.jar")`
#[macro_export]
macro_rules! local {
    ($path:expr) => {
        $crate::core::types::source::ManifestSource::Local(::std::path::PathBuf::from($path))
    };
}

/// `addon!(mod => loader: LoaderKind, source: source_expr)`
#[macro_export]
macro_rules! addon {
    (mod => loader: $loader:expr, source: $source:expr $(,)?) => {
        $crate::core::types::addon::ManifestAddon::Mod {
            loader: $loader,
            source: $source,
        }
    };
    (plugin => loader: $loader:expr, source: $source:expr $(,)?) => {
        $crate::core::types::addon::ManifestAddon::Plugin {
            loader: $loader,
            source: $source,
        }
    };
}

/// `manifest!(name: "my-server", loader: Loader::Vanilla { game_version: "1.21.4".into() }, deps: { "sodium" => addon!(mod => ...) })`
#[macro_export]
macro_rules! manifest {
    (
        name: $name:expr,
        loader: $loader:expr,
        java: $java:expr,
        deps: {
            $($key:expr => $addon:expr),* $(,)?
        }
    ) => {{
        let mut deps = ::std::collections::BTreeMap::new();
        $( deps.insert($key.into(), $addon); )*
        $crate::core::schema::manifest::ConduitManifest {
            name: $name.into(),
            java_min_version: Some($java.into()),
            loader: $loader,
            dependencies: deps,
        }
    }};
    (
        name: $name:expr,
        loader: $loader:expr,
        deps: {
            $($key:expr => $addon:expr),* $(,)?
        }
    ) => {{
        let mut deps = ::std::collections::BTreeMap::new();
        $( deps.insert($key.into(), $addon); )*
        $crate::core::schema::manifest::ConduitManifest {
            name: $name.into(),
            java_min_version: None,
            loader: $loader,
            dependencies: deps,
        }
    }};
    (name: $name:expr, loader: $loader:expr, java: $java:expr) => {
        manifest!(name: $name, loader: $loader, java: $java, deps: { })
    };
    (name: $name:expr, loader: $loader:expr) => {
        manifest!(name: $name, loader: $loader, deps: { })
    };
}

/// `lockfile!(lock_version: 1, java: "21", loader: ..., deps: { ... })`
#[macro_export]
macro_rules! lockfile {
    (
        lock_version: $lock_version:expr,
        java: $java:expr,
        loader: $loader:expr,
        deps: {
            $($key:expr => $addon:expr),* $(,)?
        }
    ) => {{
        let mut deps = ::std::collections::BTreeMap::new();
        $( deps.insert($key.into(), $addon); )*
        $crate::core::schema::lockfile::ConduitLockfile {
            lock_version: $lock_version,
            java_min_version: Some($java.into()),
            loader: $loader,
            dependencies: deps,
        }
    }};
    (
        lock_version: $lock_version:expr,
        loader: $loader:expr,
        deps: {
            $($key:expr => $addon:expr),* $(,)?
        }
    ) => {{
        let mut deps = ::std::collections::BTreeMap::new();
        $( deps.insert($key.into(), $addon); )*
        $crate::core::schema::lockfile::ConduitLockfile {
            lock_version: $lock_version,
            java_min_version: None,
            loader: $loader,
            dependencies: deps,
        }
    }};
    (lock_version: $lock_version:expr, java: $java:expr, loader: $loader:expr) => {
        lockfile!(lock_version: $lock_version, java: $java, loader: $loader, deps: { })
    };
    (lock_version: $lock_version:expr, loader: $loader:expr) => {
        lockfile!(lock_version: $lock_version, loader: $loader, deps: { })
    };
}
