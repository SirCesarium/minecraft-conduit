pub mod fmt;

use std::{io, path::Path};

pub fn get_dir_name(dir: &Path) -> io::Result<String> {
    Ok(dir
        .canonicalize()?
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default())
}

pub fn build_http_client() -> reqwest::Result<reqwest::Client> {
    reqwest::Client::builder()
        .user_agent(format!(
            "{}/{}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        ))
        .build()
}
