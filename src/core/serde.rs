use std::fmt;

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::core::types::source::VersionConstraint;

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

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a version string like \"1.0.0\" or \"latest\"")
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<VersionConstraint, E> {
                match v {
                    "latest" | "*" => Ok(VersionConstraint::Latest),
                    s => Ok(VersionConstraint::Exact(s.into())),
                }
            }
        }

        deserializer.deserialize_str(VersionVisitor)
    }
}
