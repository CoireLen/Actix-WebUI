use actix_web::{web, App, HttpServer, Responder,get,post,HttpResponse,Error};
use actix_files::{NamedFile};
use serde::{Deserialize, Serialize};
use actix_files as fs;
use std::io::Result;
use std::path::PathBuf;

#[get("/")]
async fn root() -> Result<NamedFile> {
    Ok(NamedFile::open(PathBuf::from("./ui/index.html"))?)
}

#[derive(Debug, Serialize, Deserialize)]
struct MyData{
    name:String
}
async fn echo(item: web::Json<MyData>) ->HttpResponse {
    println!("post:{}",item.0.name);
    HttpResponse::Ok().json(item.0)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ip="127.0.0.1";
    let port=8080;
    println!("Server Run in {}:{}",ip,port);
    HttpServer::new(|| {
        App::new()
            .service(root)
            .service(fs::Files::new("/link","./link").show_files_listing())
            .service(fs::Files::new("/link/ui","./link/ui").show_files_listing())
            .service(
                web::scope("/api")
                .service(web::resource("/echo").route(web::post().to(echo)))
        )
    })
    .bind((ip, port))?
    .run()
    .await
}