use std::future::Future;

use futures::future::join_all;
use minecraft_registry_api::error::ApiError;
use minecraft_registry_api::paper::models::ProjectQuery;
use minecraft_registry_api::paper::PaperClient;

use super::VersionProvider;

use super::{cmp_versions, is_stable_version};

pub struct PaperProvider;

impl VersionProvider for PaperProvider {
    fn fetch_game_versions(&self, http: &reqwest::Client) -> impl Future<Output = Result<Vec<String>, ApiError>> + Send {
        let http = http.clone();
        async move {
            let paper = PaperClient::new(http);
            let project = paper.get_project(ProjectQuery { project: "paper" }).await?;
            let mut versions: Vec<String> = project
                .versions
                .into_values()
                .flatten()
                .filter(|v| is_stable_version(v))
                .collect();
            versions.sort_by(|a, b| cmp_versions(b, a));

            let checks: Vec<_> = versions
                .iter()
                .map(|v| paper.get_builds_by_channel("paper", v, "STABLE"))
                .collect();
            let results = join_all(checks).await;
            let stable_versions: Vec<String> = versions
                .into_iter()
                .zip(results)
                .filter_map(|(v, r)| r.ok().filter(|b| !b.is_empty()).map(|_| v))
                .collect();
            Ok(stable_versions)
        }
    }

    fn fetch_loader_versions(
        &self,
        http: &reqwest::Client,
        game_version: &str,
    ) -> impl Future<Output = Result<Vec<String>, ApiError>> + Send {
        let http = http.clone();
        let game_version = game_version.to_string();
        async move {
            let paper = PaperClient::new(http);
            let builds = paper
                .get_builds_by_channel("paper", &game_version, "STABLE")
                .await?;
            Ok(builds.into_iter().map(|b| b.id.to_string()).collect())
        }
    }
}
