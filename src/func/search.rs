use crate::core::model::loader::LoaderKind;
use crate::core::provider::addon::mods::ModrinthModProvider;
use crate::core::provider::addon::{AddonProvider, SearchParams, SearchResults};
use crate::errors::{Error, Result};
use crate::utils::build_http_client;

pub struct SearchInput<'a> {
    pub query: &'a str,
    pub loader: Option<&'a str>,
    pub game_version: Option<&'a str>,
    pub loader_version: Option<&'a str>,
    pub page: usize,
    pub per_page: usize,
}

pub struct SearchOutput {
    pub results: SearchResults,
    pub page: usize,
    pub per_page: usize,
    pub total_pages: usize,
}

fn parse_loader(s: &str) -> Option<LoaderKind> {
    match s {
        "fabric" => Some(LoaderKind::Fabric),
        "forge" => Some(LoaderKind::Forge),
        "neoforge" => Some(LoaderKind::NeoForge),
        "paper" => Some(LoaderKind::Paper),
        "purpur" => Some(LoaderKind::Purpur),
        _ => None,
    }
}

pub async fn search(input: SearchInput<'_>) -> Result<SearchOutput> {
    if input.per_page == 0 || input.per_page > 100 {
        return Err(Error::InvalidArgument(
            "--per-page must be between 1 and 100".into(),
        ));
    }
    if input.page == 0 {
        return Err(Error::InvalidArgument(
            "--page must be 1 or greater".into(),
        ));
    }

    let http = build_http_client()?;
    let loader = input.loader.and_then(parse_loader);
    let offset = (input.page - 1) * input.per_page;

    let provider = ModrinthModProvider;
    let results = provider
        .search(SearchParams {
            query: input.query,
            loader,
            game_version: input.game_version,
            loader_version: input.loader_version,
            limit: input.per_page,
            offset,
            http: &http,
        })
        .await?;

    let total_pages = (results.total as f64 / input.per_page as f64).ceil() as usize;

    Ok(SearchOutput {
        results,
        page: input.page,
        per_page: input.per_page,
        total_pages,
    })
}
