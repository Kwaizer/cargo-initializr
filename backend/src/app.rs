use crate::handlers::{download, starters};
use actix_web::{App, HttpServer};
use std::io;
use tracing_actix_web::TracingLogger;

pub async fn start_up() -> io::Result<()> {
    let host = dotenv::var("HOST").unwrap();
    let port = dotenv::var("PORT").unwrap().parse().unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(download)
            .service(starters)
    })
    .bind((host, port))?
    .run()
    .await
}
