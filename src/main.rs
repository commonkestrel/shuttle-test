use actix_web::{get, HttpResponse, HttpRequest, Responder, web::{self, ServiceConfig}};
use shuttle_actix_web::ShuttleActixWeb;
use std::{io::ErrorKind, fs, path::Path};

#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Yippee")
}

#[get("/static/{path}")]
async fn serve_static(req: HttpRequest, path: web::Path<String>) -> HttpResponse {
    let f = match actix_files::NamedFile::open_async(Path::new("static").join(&path.into_inner())).await {
        Ok(f) => f,
        Err(err) => return match err.kind() {
            ErrorKind::NotFound => HttpResponse::NotFound().into(),
            ErrorKind::InvalidInput => HttpResponse::BadRequest().into(),
            _ => HttpResponse::InternalServerError().into(),
        }
    };

    f.into_response(&req)
}

#[shuttle_runtime::main]
async fn actix_web(
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
        cfg.service(serve_static);
    };

    Ok(config.into())
}
