use std::future::Future;

use futures::future::join_all;
use minecraft_registry_api::error::ApiError;
use minecraft_registry_api::purpur::models::VersionQuery;
use minecraft_registry_api::purpur::PurpurClient;

use super::VersionProvider;

use super::{cmp_versions, is_stable_version};

pub struct PurpurProvider;

impl VersionProvider for PurpurProvider {
    fn fetch_game_versions(&self, http: &reqwest::Client) -> impl Future<Output = Result<Vec<String>, ApiError>> + Send {
        let http = http.clone();
        async move {
            let purpur = PurpurClient::new(http);
            let project = purpur.get_project().await?;
            let mut versions: Vec<String> = project
                .versions
                .into_iter()
                .filter(|v| is_stable_version(v))
                .collect();
            versions.sort_by(|a, b| cmp_versions(b, a));

            let version_checks: Vec<_> = versions
                .iter()
                .map(|v| purpur.get_version(VersionQuery { version: v }))
                .collect();
            let version_results = join_all(version_checks).await;
            let mut builds: Vec<(String, String)> = Vec::new();
            for (v, r) in versions.iter().zip(version_results) {
                if let Ok(info) = r {
                    builds.push((v.clone(), info.builds.latest));
                }
            }
            let build_checks: Vec<_> = builds
                .iter()
                .map(|(version, build)| purpur.get_build(version, build))
                .collect();
            let build_results = join_all(build_checks).await;
            let stable_versions: Vec<String> = builds
                .into_iter()
                .zip(build_results)
                .filter_map(|((version, _), r)| {
                    r.ok().and_then(|b| {
                        let is_exp = b
                            .metadata
                            .and_then(|m| m.type_field)
                            .is_some_and(|t| t == "experimental");
                        (!is_exp).then_some(version)
                    })
                })
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
            let purpur = PurpurClient::new(http);
            let version = purpur
                .get_version(VersionQuery { version: &game_version })
                .await?;
            let mut builds = version.builds.all;
            builds.sort_by(|a, b| b.cmp(a));

            let build_checks: Vec<_> = builds
                .iter()
                .map(|b| purpur.get_build(&game_version, b))
                .collect();
            let results = join_all(build_checks).await;
            let stable_builds: Vec<String> = builds
                .into_iter()
                .zip(results)
                .filter_map(|(b, r)| {
                    r.ok().and_then(|info| {
                        let is_exp = info
                            .metadata
                            .and_then(|m| m.type_field)
                            .is_some_and(|t| t == "experimental");
                        (!is_exp).then_some(b)
                    })
                })
                .collect();
            Ok(stable_builds)
        }
    }
}
