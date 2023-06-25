use common::project_description_dto::ProjectDescriptionDto;

use actix_web::http::header::{ContentDisposition, ContentType};
use actix_web::web::Json;
use actix_web::Error as ActixError;
use actix_web::{get, HttpResponse, Responder};

use crate::service::project_generation_service::ProjectGeneratingServiceError;
use crate::service::starter_service::StarterServiceError;
use crate::service::{project_generation_service, starter_service};
use futures::future::ok;

const INVALID_FILES_ERROR_MASSAGE: &str =
    "Unfortunately something went wrong and ours files are invalid.";
const DAMAGED_FILES_ERROR_MASSAGE: &str =
    "Unfortunately something went wrong and ours files are damaged.";
const NOTHING_TO_OFFER_ERROR_MASSAGE: &str =
    "Unfortunately something went wrong and we have nothing to offer you :(";
const GENERATION_FAILURE_ERROR_MASSAGE: &str =
    "Unfortunately something went wrong and we cannot generate this project.";

#[get("/starters")]
pub async fn starters() -> impl Responder {
    match starter_service::get_starters() {
        Ok(starters) => HttpResponse::Ok().json(starters),
        Err(e) => match &e {
            StarterServiceError::InvalidStarterManifest(_) => {
                tracing::error!("{e:?}");

                HttpResponse::InternalServerError().json(INVALID_FILES_ERROR_MASSAGE)
            }
            StarterServiceError::NoStartersProvided => {
                tracing::error!("{e:?}");

                HttpResponse::NotImplemented().json(NOTHING_TO_OFFER_ERROR_MASSAGE)
            }
            StarterServiceError::CannotReadMetadataOfStarterFile(_) => {
                tracing::error!("{e:?}");

                HttpResponse::InternalServerError().json(DAMAGED_FILES_ERROR_MASSAGE)
            }
            StarterServiceError::MissingPackageSection(_) => {
                tracing::error!("{e:?}");

                HttpResponse::VariantAlsoNegotiates().json(INVALID_FILES_ERROR_MASSAGE)
            }
        },
    }
}

#[get("/download")]
pub async fn download(description_dto: Json<ProjectDescriptionDto>) -> impl Responder {
    let original_project_name = &description_dto.package_description.name;

    let buffered_project = match project_generation_service::generate(&description_dto.0) {
        Ok(buffered_project) => buffered_project,
        Err(e) => {
            return match &e {
                ProjectGeneratingServiceError::DescriptionSection(_) => {
                    tracing::error!("{e:?}");

                    HttpResponse::InternalServerError().json(GENERATION_FAILURE_ERROR_MASSAGE)
                }
                ProjectGeneratingServiceError::DependencySection(_) => {
                    tracing::error!("{e:?}");

                    HttpResponse::InternalServerError().json(GENERATION_FAILURE_ERROR_MASSAGE)
                }
                ProjectGeneratingServiceError::CouldNotGetStarterContent(_) => {
                    tracing::error!("{e:?}");

                    HttpResponse::VariantAlsoNegotiates().json(DAMAGED_FILES_ERROR_MASSAGE)
                }
                ProjectGeneratingServiceError::CouldNotParseStarterManifest(_) => {
                    tracing::error!("{e:?}");

                    HttpResponse::VariantAlsoNegotiates().json(DAMAGED_FILES_ERROR_MASSAGE)
                }
                ProjectGeneratingServiceError::CompressionError(_) => {
                    tracing::error!("{e:?}");

                    HttpResponse::InternalServerError().json(GENERATION_FAILURE_ERROR_MASSAGE)
                }
                ProjectGeneratingServiceError::IoError(_) => {
                    tracing::error!("{e:?}");

                    HttpResponse::InternalServerError().json(GENERATION_FAILURE_ERROR_MASSAGE)
                }
                ProjectGeneratingServiceError::ProjectError(_) => {
                    tracing::error!("{e:?}");

                    HttpResponse::InternalServerError().json(GENERATION_FAILURE_ERROR_MASSAGE)
                }
            }
        }
    };

    let bytes = actix_web::web::Bytes::from(buffered_project);
    let stream = futures::stream::once(ok::<_, ActixError>(bytes));

    HttpResponse::Ok()
        .content_type(ContentType::octet_stream())
        .insert_header(ContentDisposition::attachment(format!(
            "{}{}",
            original_project_name, ".zip"
        )))
        .streaming(stream)
}
