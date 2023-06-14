use crate::project_description_dto::ProjectDescriptionDto;

use actix_web::http::header::{ContentDisposition, ContentType};
use actix_web::web::Json;
use actix_web::Error as ActixError;
use actix_web::{get, HttpResponse, Responder};

use crate::service::starter_service::StarterServiceError;
use crate::service::{project_generator_service, starter_service};
use futures::future::ok;

const INVALID_FILES_ERROR_MASSAGE: &str =
    "Unfortunately something went wrong and ours files are invalid.";
const DAMAGED_FILES_ERROR_MASSAGE: &str =
    "Unfortunately something went wrong and ours files are damaged.";
const NOTHING_TO_OFFER_ERROR_MASSAGE: &str =
    "Unfortunately something went wrong and we have nothing to offer you :(";

#[get("/starters")]
pub async fn starters() -> impl Responder {
    match starter_service::get_starters() {
        Ok(starters) => HttpResponse::Ok().json(starters),
        Err(e) => match &e {
            StarterServiceError::InvalidStarterManifest(_) => {
                tracing::error!("{e:?}");

                HttpResponse::InternalServerError().body(INVALID_FILES_ERROR_MASSAGE)
            }
            StarterServiceError::NoStartersProvided => {
                tracing::error!("{e:?}");

                HttpResponse::NotImplemented().body(NOTHING_TO_OFFER_ERROR_MASSAGE)
            }
            StarterServiceError::CannotReadMetadataOfStarterFile(_) => {
                tracing::error!("{e:?}");

                HttpResponse::InternalServerError().body(DAMAGED_FILES_ERROR_MASSAGE)
            }
            StarterServiceError::MissingPackageSection(_) => {
                tracing::error!("{e:?}");

                HttpResponse::VariantAlsoNegotiates().body(INVALID_FILES_ERROR_MASSAGE)
            }
        },
    }
}

#[get("/download")]
pub async fn download(description_dto: Json<ProjectDescriptionDto>) -> impl Responder {
    let original_project_name = &description_dto.package_description.name;

    let buffered_project = match project_generator_service::generate(&description_dto.0) {
        Ok(buffered_project) => buffered_project,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    let bytes = actix_web::web::Bytes::from(buffered_project);
    let stream = futures::stream::once(ok::<_, ActixError>(bytes));

    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .insert_header(ContentDisposition::attachment(format!(
            "{}{}",
            original_project_name, ".zip"
        )))
        .streaming(stream)
}
