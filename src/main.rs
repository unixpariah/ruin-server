use actix_web::{get, http::StatusCode, web, App, HttpResponse, HttpServer, Responder};
use std::fs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_image))
        .bind(("0.0.0.0", 5626))?
        .run()
        .await
}

#[get("/{filename}")]
async fn get_image(filename: web::Path<String>) -> impl Responder {
    let path = format!("./images/{filename}.png");
    let image = web::block(|| match fs::read(path) {
        Ok(image) => image,
        Err(_) => fs::read(format!("./images/linux.png")).unwrap(),
    })
    .await
    .unwrap();
    println!("get request /{filename} received");
    HttpResponse::build(StatusCode::OK)
        .content_type("image/png")
        .body(image)
}
