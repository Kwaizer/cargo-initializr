use std::collections::HashMap;
use std::future::{ready, Ready};

use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use common::project_description_dto::package_description::{
    PackageDescription,
    PackageDescriptionName,
};
use common::project_description_dto::target_kind::TargetKind;
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
                let (k, v) = kv.split_once("=").unwrap();
                (k.to_owned(), v.to_owned())
            })
            .collect::<HashMap<String, String>>();

        let target_kind = params
            .get("target_kind")
            .map_or(TargetKind::default(), |t| t.as_str().into());
        let package_description = PackageDescription {
            name:        params
                .get("name")
                .map_or(PackageDescriptionName::default(), |n| {
                    PackageDescriptionName(n.clone())
                }),
            author:      params.get("author").cloned(),
            description: params.get("description").cloned(),
        };
        let starters = params.get("starters").map_or(Vec::default(), |starters| {
            let starters = starters.clone();
            let starters = &starters[1..starters.len() - 1]; // remove first and last character
            starters.split(";").map(|s| s.to_owned()).collect()
        });

        let project_description_dto = ProjectDescriptionDto {
            target_kind,
            package_description,
            starters,
        };

        let download_payload = DownloadPayload {
            project_description_dto,
        };

        log::debug!("{download_payload:#?}");

        ready(Ok(download_payload))
    }
}
