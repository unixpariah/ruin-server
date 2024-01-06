use actix_web::{
    get,
    http::StatusCode,
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};
use shuttle_actix_web::ShuttleActixWeb;
use std::fs;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(get_image);
    };

    Ok(config.into())
}

#[get("/{filename}")]
async fn get_image(filename: web::Path<String>) -> impl Responder {
    let path = format!("./images/{filename}.png");
    let image = web::block(|| match fs::read(path) {
        Ok(image) => image,
        Err(_) => fs::read(format!("images/linux.png")).unwrap(),
    })
    .await
    .unwrap();
    println!("get request /{filename} received");
    HttpResponse::build(StatusCode::OK)
        .content_type("image/png")
        .body(image)
}
