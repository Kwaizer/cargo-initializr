use crate::project_description_dto::ProjectDescriptionDto;

use actix_web::http::header::{ContentDisposition, ContentType};
use actix_web::web::Json;
use actix_web::Error as ActixError;
use actix_web::{get, HttpResponse, Responder};

use futures::future::ok;

#[get("/starters")]
pub async fn starters() -> impl Responder {
    match crate::starter_service::get_starters() {
        Ok(starters) => HttpResponse::Ok().json(starters),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/download")]
pub async fn download(description_dto: Json<ProjectDescriptionDto>) -> impl Responder {
    let original_project_name = &description_dto.package_description.name;

    let buffered_project = match crate::generate_service::generate(&description_dto.0) {
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
