use std::collections::HashMap;
use std::future::{ready, Ready};

use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use common::project_description_dto::package_description::{
    PackageDescription,
    PackageDescriptionName,
};
use common::project_description_dto::ProjectDescriptionDto;
use serde::{Deserialize, Serialize};
use tracing::log;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct DownloadPayload {
    pub project_description_dto: ProjectDescriptionDto,
}

impl FromRequest for DownloadPayload {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let params = req
            .query_string()
            .split("&")
            .into_iter()
            .map(|kv| {
                kv.split_once("=")
                    .map(|(k, v)| (k.to_owned(), v.to_owned()))
                    .unwrap()
            })
            .collect::<HashMap<String, String>>();

        let target_kind = params
            .get("target_kind")
            .map(|t| t.as_str().into())
            .unwrap_or_default();
        let package_description = PackageDescription {
            name:        params
                .get("name")
                .map_or(PackageDescriptionName::default(), |n| {
                    PackageDescriptionName(n.clone())
                }),
            author:      params.get("author").cloned(),
            description: params.get("description").cloned(),
        };
        let starters = params
            .get("starters")
            .map(|starters| {
                remove_first_and_last_character(&starters)
                    .split(";")
                    .map(|s| s.to_owned())
                    .collect()
            })
            .unwrap_or_default();

        let download_payload = DownloadPayload {
            project_description_dto: ProjectDescriptionDto {
                target_kind,
                package_description,
                starters,
            },
        };

        log::debug!("{download_payload:#?}");

        ready(Ok(download_payload))
    }
}

fn remove_first_and_last_character(str: &str) -> String {
    str[1..str.len() - 1].to_owned()
}
