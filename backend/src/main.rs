use std::path::PathBuf;
use std::str::FromStr;
use std::{fs, io};

mod app;
mod cargo_toml_parser_extensions;
mod extractor;
mod handlers;
mod logging;
mod macros;
mod repository;
mod service;
mod storage;

#[actix_web::main]
async fn main() -> io::Result<()> {
    check_env_vars();

    logging::set_up_logging();

    app::start_up().await
}

fn check_env_vars() {
    dotenv::from_path("./backend/.env").expect("Failed to read .env file");

    let temp = dotenv::var("TEMP").expect("Missing TEMP variable");
    let temp = PathBuf::from_str(&temp).expect("invalid TEMP variable");
    if !temp.exists() {
        fs::create_dir(temp).expect("TEMP folder creation failure")
    }

    let content = dotenv::var("CONTENT").expect("Missing CONTENT variable");
    PathBuf::from_str(&content).expect("Invalid CONTENT variable");

    dotenv::var("HOST").expect("Invalid HOST variable");

    let port = dotenv::var("PORT").expect("Missing PORT variable");
    port.parse::<u16>().expect("Invalid PORT variable");

    dotenv::var("LOG_LEVEL").expect("Missing LOG_LEVEL variable");

    dotenv::var("LABEL").expect("Missing LABEL variable");

    let storage_mode = dotenv::var("STORAGE_MODE").expect("Missing STORAGE_MODE variable");
    if !storage_mode.eq_ignore_ascii_case("FS") && !storage_mode.eq_ignore_ascii_case("REDIS") {
        panic!("Invalid STORAGE_MODE variable")
    }
}
