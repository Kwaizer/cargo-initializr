use std::collections::HashMap;
use std::io;

use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use common::starter::Starter;
use tracing_actix_web::TracingLogger;

use crate::handlers::{download, starters};
use crate::service::starter_service::StarterService;

pub async fn start_up() -> io::Result<()> {
    let host = dotenv::var("HOST").unwrap();
    let port = dotenv::var("PORT").unwrap().parse().unwrap();
    let storage_mode = dotenv::var("STORAGE_MODE").unwrap();
    let starter_service = init_state(storage_mode).await;

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(TracingLogger::default())
            .app_data(Data::new(starter_service.clone()))
            .service(download)
            .service(starters)
    })
    .bind((host, port))?
    .run()
    .await
}

async fn init_state(_: String) -> StarterService {
    let map: HashMap<String, Starter> = HashMap::new();

    StarterService::new(map).await
}
