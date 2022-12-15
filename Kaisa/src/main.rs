use actix_web::{web, App, HttpServer, Responder,get,post,HttpResponse,Error};
use actix_files::{NamedFile};
use serde::{Deserialize, Serialize};
use actix_files as fs;
use std::io::Result;
use std::path::PathBuf;

#[get("/")]
async fn root() -> Result<NamedFile> {
    println!("GET /");
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
#[derive(Debug, Serialize, Deserialize)]
struct KaisaData{
    kaisa:String,
    offset:String,
}
#[derive(Debug, Serialize, Deserialize)]
struct KaisaoutData{
    kaisaout:String,
}
async fn kaisa(item: web::Json<KaisaData>) ->HttpResponse {
    let mut vdata=String::new();
    let offset=item.0.offset.parse::<i32>().unwrap();
    for i in item.0.kaisa.as_bytes(){
        if (*i >='A' as u8) &&(*i<='Z' as u8){
            let tmp=i-'A' as u8;
            vdata.push((((tmp as i32+offset)%26) as u8 +'A' as u8) as char);
        }
        else  if (*i >='a' as u8 )&& (*i<='z' as u8){
            let tmp=i-'a' as u8;
            vdata.push((((tmp as i32+offset)%26) as u8 +'a' as u8) as char);
        }
        else{
            vdata.push(*i as char );
        }
    }
    println!("/api/kaisa post:{} | {}",item.0.kaisa,vdata);
    HttpResponse::Ok().json(web::Json(KaisaoutData{kaisaout:vdata}))
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ip="127.0.0.1";
    let port=1080;
    println!("Server Run in {}:{}",ip,port);
    HttpServer::new(|| {
        App::new()
            .service(root)
            .service(fs::Files::new("/link","./link").show_files_listing())
            //.service(fs::Files::new("/link/ui","./link/ui").show_files_listing())
            .service(
                web::scope("/api")
                .service(web::resource("/echo").route(web::post().to(echo)))
                .service(web::resource("/kaisa").route(web::post().to(kaisa)))
        )
    })
    .bind((ip, port))?
    .run()
    .await
}