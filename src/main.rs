use actix_files::NamedFile;
use actix_web::{web, App, HttpRequest, HttpServer, Result};
use dotenv::dotenv;
use nix::unistd::{fork, ForkResult};
use std::env::var;
use std::path::PathBuf;

async fn index(req: HttpRequest) -> Result<NamedFile> {
    let base_url = var("STATIC_PATH").expect("localhost null");
    let raw = req.match_info().query("filename");
    let path: PathBuf = format!("{}{}", base_url, raw).parse().unwrap();
    Ok(NamedFile::open(path)?)
}

fn main() {
    match unsafe { fork() } {
        Ok(ForkResult::Parent { .. }) => {
            println!("静态服务器启动成功")
        }
        Ok(ForkResult::Child) => {
            let _ = start();
        }
        Err(_) => println!("Fork failed"),
    }
}

#[actix_web::main]
async fn start() -> std::io::Result<()> {
    dotenv().ok();
    let local = var("LOCALHOST").expect("localhost null");
    HttpServer::new(move || App::new().route("/{filename:.*}", web::get().to(index)))
        .bind(local)?
        .run()
        .await
}
