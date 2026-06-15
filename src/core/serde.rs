use std::fmt;

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::core::types::source::VersionConstraint;

impl Serialize for VersionConstraint {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Exact(v) => serializer.serialize_str(v),
            Self::Latest => serializer.serialize_str("latest"),
            Self::Range(v) => serializer.serialize_str(v),
        }
    }
}

fn is_inexact(s: &str) -> bool {
    s.starts_with('^')
        || s.starts_with('~')
        || s.starts_with('>')
        || s.starts_with('<')
        || s.starts_with('=')
        || s.contains(',')
        || s.contains('x')
        || s.contains('X')
        || s.contains('*')
}

impl<'de> Deserialize<'de> for VersionConstraint {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct VersionVisitor;

        impl Visitor<'_> for VersionVisitor {
            type Value = VersionConstraint;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a version string like \"1.0.0\", \"latest\", or \">=1.21\"")
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<VersionConstraint, E> {
                match v {
                    "latest" | "*" => Ok(VersionConstraint::Latest),
                    s if is_inexact(s) => Ok(VersionConstraint::Range(s.into())),
                    s => Ok(VersionConstraint::Exact(s.into())),
                }
            }
        }

        deserializer.deserialize_str(VersionVisitor)
    }
}
