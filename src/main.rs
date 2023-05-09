use std::io;
use std::path::PathBuf;
use std::str::FromStr;

mod app;
mod compressor;
mod generate_service;
mod handlers;
mod logging;
mod macros;
mod project_description_dto;
mod starter_service;

#[actix_web::main]
async fn main() -> io::Result<()> {
    check_env_vars();

    logging::set_up_logging();

    app::start_up().await
}

fn check_env_vars() {
    let temp = dotenv::var("TEMP").expect("Missing TEMP variable");
    PathBuf::from_str(&temp).expect("invalid TEMP variable");

    let content = dotenv::var("CONTENT").expect("Missing CONTENT variable");
    PathBuf::from_str(&content).expect("Invalid CONTENT variable");

    dotenv::var("HOST").expect("Invalid HOST variable");

    let port = dotenv::var("PORT").expect("Missing PORT variable");
    port.parse::<u16>().expect("Invalid PORT variable");

    dotenv::var("LOG_LEVEL").expect("Missing LOG_LEVEL variable");
}
