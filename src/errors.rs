use std::io;
use std::path::PathBuf;
use std::result;

use minecraft_registry_api::error::ApiError;
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
    #[error("this source does not support search")]
    SearchNotSupported,
    #[error("api error: {0}")]
    Api(#[from] ApiError),
    #[error("addon not found: {0}")]
    AddonNotFound(String),
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    InvalidArgument(String),
}
