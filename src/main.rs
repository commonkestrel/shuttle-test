use actix_web::{get, HttpResponse, HttpRequest, Responder, web::{self, ServiceConfig}};
use shuttle_actix_web::ShuttleActixWeb;
use std::{io::ErrorKind, path::Path};

#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Yippee")
}

#[shuttle_runtime::main]
async fn actix_web(
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
        cfg.service(actix_files::Files::new("/static", "./static").show_files_listing());
    };

    Ok(config.into())
}
