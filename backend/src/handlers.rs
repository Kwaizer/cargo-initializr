use actix_web::http::header::{ContentDisposition, ContentType};
use actix_web::web::{Data, Json};
use actix_web::{get, Error as ActixError, HttpResponse, Responder};
use common::project_description_dto::ProjectDescriptionDto;
use futures::future::ok;

use crate::service::project_generation_service;
use crate::service::project_generation_service::ProjectGeneratingServiceError;
use crate::service::starter_service::{StarterService, StarterServiceError};
use crate::service::traits::StarterRepository;

const INVALID_FILES_ERROR_MASSAGE: &str =
    "Unfortunately something went wrong and ours files are invalid.";
const DAMAGED_FILES_ERROR_MASSAGE: &str =
    "Unfortunately something went wrong and ours files are damaged.";
const NOTHING_TO_OFFER_ERROR_MASSAGE: &str =
    "Unfortunately something went wrong and we have nothing to offer you :(";
const GENERATION_FAILURE_ERROR_MASSAGE: &str =
    "Unfortunately something went wrong and we cannot generate this project.";

#[get("/starters")]
pub async fn starters(starter_service: Data<StarterService>) -> impl Responder {
    match starter_service.get_starters().await {
        Ok(starters) => HttpResponse::Ok().json(starters),
        Err(e) =>
            match &e {
                StarterServiceError::InvalidStarterManifest(_) => {
                    tracing::error!("{e:?}");

                    HttpResponse::InternalServerError().json(INVALID_FILES_ERROR_MASSAGE)
                },
                StarterServiceError::NoStartersProvided => {
                    tracing::error!("{e:?}");

                    HttpResponse::NotImplemented().json(NOTHING_TO_OFFER_ERROR_MASSAGE)
                },
                StarterServiceError::CannotReadMetadataOfStarterFile(_) => {
                    tracing::error!("{e:?}");

                    HttpResponse::InternalServerError().json(DAMAGED_FILES_ERROR_MASSAGE)
                },
                StarterServiceError::MissingPackageSection(_) => {
                    tracing::error!("{e:?}");

                    HttpResponse::VariantAlsoNegotiates().json(INVALID_FILES_ERROR_MASSAGE)
                },
                StarterServiceError::CannotReadStarterFile(_) => {
                    tracing::error!("{e:?}");

                    HttpResponse::VariantAlsoNegotiates().json(INVALID_FILES_ERROR_MASSAGE)
                },
            },
    }
}

#[get("/download")]
pub async fn download(
    description_dto: Json<ProjectDescriptionDto>,
    starter_service: Data<StarterService>,
) -> impl Responder {
    let buffered_project =
        match project_generation_service::generate(&description_dto.0, starter_service.get_ref())
            .await
        {
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
                    ProjectGeneratingServiceError::StarterServiceError(_) => {
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
