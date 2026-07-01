use minecraft_registry_api::modrinth::models::{Project, ProjectRef, Version, VersionListQuery};
use minecraft_registry_api::modrinth::ModrinthClient;

use crate::errors::Result;
use crate::utils::build_http_client;

pub async fn fetch_info(slug: &str) -> Result<(Project, Vec<Version>)> {
    let http = build_http_client()?;
    let client = ModrinthClient::new(http);

    let project = client
        .get_project(ProjectRef { slug })
        .await?;

    let mut versions = client
        .get_versions(VersionListQuery {
            project_id: &project.id,
        })
        .await?;

    versions.sort_by(|a, b| b.date_published.cmp(&a.date_published));

    Ok((project, versions))
}
