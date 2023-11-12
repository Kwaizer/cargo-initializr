use actix_web::http::header::{ContentDisposition, ContentType};
use actix_web::web::Data;
use actix_web::{get, Error as ActixError, HttpResponse, Responder};
use common::starter::starter_dto::StarterDto;
use futures::future::ok;

use crate::app::AppContext;
use crate::extractor::DownloadPayload;
use crate::service::project_generation_service;
use crate::service::project_generation_service::ProjectGeneratingServiceError;
use crate::storage::errors::MapStorageError;

const INVALID_FILES_ERROR_MASSAGE: &str =
    "Unfortunately something went wrong and ours files are invalid.";
const DAMAGED_FILES_ERROR_MASSAGE: &str =
    "Unfortunately something went wrong and ours files are damaged.";
const GENERATION_FAILURE_ERROR_MASSAGE: &str =
    "Unfortunately something went wrong and we cannot generate this project.";
const CANNOT_FIND_FILES_ERROR_MASSAGE: &str =
    "Unfortunately something went wrong and we cannot find starter.";
const UNKNOWN_ERROR_MASSAGE: &str = "Unfortunately something went wrong.";

#[get("/starters")]
pub async fn starters(ctx: Data<AppContext>) -> impl Responder {
    match ctx.starter_service.get_starters().await {
        Ok(starters) => {
            let starters_dto = starters
                .into_iter()
                .map(|starter| starter.starter_dto)
                .collect::<Vec<StarterDto>>();
            HttpResponse::Ok().json(starters_dto)
        },
        Err(e) =>
            match &e {
                MapStorageError::Unknown(unknown_error) => {
                    tracing::error!("{unknown_error}: {e:?}");

                    HttpResponse::InternalServerError().json(UNKNOWN_ERROR_MASSAGE)
                },
                MapStorageError::DeserializeFailed(_) => {
                    tracing::error!("{e:?}");

                    HttpResponse::InternalServerError().json(INVALID_FILES_ERROR_MASSAGE)
                },
                MapStorageError::SerializeFailed(_) => {
                    tracing::error!("{e:?}");

                    HttpResponse::InternalServerError().json(INVALID_FILES_ERROR_MASSAGE)
                },
                MapStorageError::KeyNotFound(key) => {
                    tracing::error!("Cannot find {key}: {e:?}");

                    HttpResponse::InternalServerError().json(CANNOT_FIND_FILES_ERROR_MASSAGE)
                },
            },
    }
}

#[get("/download")]
pub async fn download(payload: DownloadPayload, ctx: Data<AppContext>) -> impl Responder {
    let description_dto = payload.project_description_dto;
    let buffered_project =
        match project_generation_service::generate(&description_dto, &ctx.starter_service).await {
            Ok(buffered_project) => buffered_project,
            Err(e) =>
                return match &e {
                    ProjectGeneratingServiceError::DescriptionSection(_) => {
                        tracing::error!("{e:?}");

                        HttpResponse::InternalServerError().json(GENERATION_FAILURE_ERROR_MASSAGE)
                    },
                    ProjectGeneratingServiceError::DependencySection(_) => {
                        tracing::error!("{e:?}");

                        HttpResponse::InternalServerError().json(GENERATION_FAILURE_ERROR_MASSAGE)
                    },
                    ProjectGeneratingServiceError::CouldNotGetStarterContent(_) => {
                        tracing::error!("{e:?}");

                        HttpResponse::VariantAlsoNegotiates().json(DAMAGED_FILES_ERROR_MASSAGE)
                    },
                    ProjectGeneratingServiceError::CouldNotParseStarterManifest(_) => {
                        tracing::error!("{e:?}");

                        HttpResponse::VariantAlsoNegotiates().json(DAMAGED_FILES_ERROR_MASSAGE)
                    },
                    ProjectGeneratingServiceError::CompressionError(_) => {
                        tracing::error!("{e:?}");

                        HttpResponse::InternalServerError().json(GENERATION_FAILURE_ERROR_MASSAGE)
                    },
                    ProjectGeneratingServiceError::IoError(_) => {
                        tracing::error!("{e:?}");

                        HttpResponse::InternalServerError().json(GENERATION_FAILURE_ERROR_MASSAGE)
                    },
                    ProjectGeneratingServiceError::ProjectError(_) => {
                        tracing::error!("{e:?}");

                        HttpResponse::InternalServerError().json(GENERATION_FAILURE_ERROR_MASSAGE)
                    },
                },
        };

    let bytes = actix_web::web::Bytes::from(buffered_project);
    let stream = futures::stream::once(ok::<_, ActixError>(bytes));

    HttpResponse::Ok()
        .content_type(ContentType::octet_stream())
        .insert_header(ContentDisposition::attachment(format!(
            "{}{}",
            &description_dto.package_description.name.0, ".zip"
        )))
        .streaming(stream)
}
