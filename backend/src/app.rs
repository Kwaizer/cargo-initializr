use std::io;

use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use tracing::log;
use tracing_actix_web::TracingLogger;

use crate::handlers::{download, starters};
use crate::service::starter_service::StarterService;
use crate::storage::in_memory_storage::InMemoryStorage;

#[derive(Clone, Debug)]
pub struct AppContext {
    pub starter_service: StarterService<InMemoryStorage>,
}

pub async fn start_up() -> io::Result<()> {
    let host = dotenv::var("HOST").unwrap();
    let port = dotenv::var("PORT").unwrap().parse().unwrap();
    let ctx = AppContext {
        starter_service: StarterService::new(
            InMemoryStorage::new()
                .map_err(|e| {
                    log::error!("{e}");
                    e
                })
                .expect("Cannot initialize 'InMemoryStorage'."),
        ),
    };

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(TracingLogger::default())
            .app_data(Data::new(ctx.clone()))
            .service(download)
            .service(starters)
    })
    .bind((host, port))?
    .run()
    .await
}
