use std::collections::BTreeMap;
use std::fmt;
use std::path::PathBuf;

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq)]
pub enum VersionConstraint {
    Exact(String),
    Latest,
}

impl Serialize for VersionConstraint {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Exact(v) => serializer.serialize_str(v),
            Self::Latest => serializer.serialize_str("latest"),
        }
    }
}

impl<'de> Deserialize<'de> for VersionConstraint {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct VersionVisitor;

        impl Visitor<'_> for VersionVisitor {
            type Value = VersionConstraint;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a version string like \"1.0.0\" or \"latest\"")
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<VersionConstraint, E> {
                match v {
                    "latest" | "*" => Ok(VersionConstraint::Latest),
                    s => Ok(VersionConstraint::Exact(s.to_string())),
                }
            }
        }

        deserializer.deserialize_str(VersionVisitor)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "source", rename_all = "snake_case")]
pub enum ManifestSource {
    Modrinth {
        slug: String,
        version: VersionConstraint,
    },
    GitHub {
        owner: String,
        repo: String,
        tag: VersionConstraint,
        file: String,
    },
    Url {
        url: String,
    },
    Local {
        path: PathBuf,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hashes {
    pub algorithms: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "source", rename_all = "snake_case")]
pub enum LockfileSource {
    Modrinth {
        project_id: String,
        version_id: String,
        filename: String,
        url: String,
        hashes: Hashes,
    },
    GitHub {
        owner: String,
        repo: String,
        tag: String,
        filename: String,
        url: String,
        hashes: Hashes,
    },
    Url {
        url: String,
        hashes: Hashes,
    },
    Local {
        filename: String,
        hashes: Hashes,
    },
}
