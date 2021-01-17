use actix_files::NamedFile;
use actix_web::{error, web, App, HttpRequest, HttpServer, Result};
use std::env::var;
use std::path::PathBuf;

async fn index(req: HttpRequest) -> Result<NamedFile> {
    let base_url = var("STATIC_PATH").expect("localhost null");
    let raw = req.match_info().query("filename");
    let path: PathBuf = format!("{}{}", base_url, raw).parse().unwrap();
    if let Ok(val) = NamedFile::open(path) {
        return Ok(val);
    } else {
        Err(error::ErrorNotFound("not found"))
    }
}

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    let local = var("LOCALHOST").expect("localhost null");
    HttpServer::new(move || App::new().route("/{filename:.*}", web::get().to(index)))
        .bind(local)?
        .run()
        .await
}
