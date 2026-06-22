use std::io;
use std::path::PathBuf;
use std::result;

use thiserror::Error;
use toml::de;
use toml::ser;

pub type Result<T> = result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to read file `{path}`")]
    Read {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("failed to write file `{path}`")]
    Write {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("failed to parse TOML in `{path}`")]
    TomlDeserialize {
        path: PathBuf,
        #[source]
        source: de::Error,
    },
    #[error("failed to serialize TOML")]
    TomlSerialize(#[from] ser::Error),
}
