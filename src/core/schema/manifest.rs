use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::core::types::{addon::ManifestAddon, loader::Loader};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConduitManifest {
    pub name: Box<str>,
    pub java: Option<Box<str>>,
    pub loader: Loader,
    pub dependencies: BTreeMap<Box<str>, ManifestAddon>,
}

impl Default for ConduitManifest {
    fn default() -> Self {
        Self {
            name: Box::from("my-server"),
            java: None,
            loader: Loader::default(),
            dependencies: BTreeMap::new(),
        }
    }
}

pub(crate) fn sanitize_name(input: &str) -> Box<str> {
    let sanitized: String = input
        .chars()
        .map(|c| match c {
            'A'..='Z' => c.to_ascii_lowercase(),
            'a'..='z' | '0'..='9' | '-' | '_' => c,
            _ => '-',
        })
        .collect();
    sanitized.into()
}

impl ConduitManifest {
    #[must_use]
    pub fn new(name: &str, loader: Loader, java: Option<Box<str>>) -> Self {
        Self {
            name: sanitize_name(name),
            java,
            loader,
            dependencies: BTreeMap::new(),
        }
    }
}
