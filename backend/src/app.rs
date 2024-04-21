use std::io;

use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use tracing_actix_web::TracingLogger;

use crate::handlers::{download, starters};
use crate::service::starter_service::StarterService;
use crate::storage::in_memory_storage::InMemoryStorage;

#[derive(Clone, Debug)]
pub struct AppContext {
    pub starter_service: StarterService<InMemoryStorage>,
}

pub async fn start_up() -> io::Result<()> {
    let host = dotenv::var("HOST").expect("Cannot get 'HOST' env var.");
    let port = dotenv::var("PORT")
        .expect("Cannot get 'PORT' env var.")
        .parse()
        .expect("Cannot parse 'PORT' env var.");
    let ctx = AppContext {
        starter_service: StarterService::in_memory(),
    };

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(TracingLogger::default())
            .app_data(Data::new(ctx.clone()))
            .service(download)
            .service(starters)
    })
    .bind((host, port))?
    .run()
    .await
}
