use actix_web::{get, guard, web, App, HttpServer, Responder, Result};
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    message: String,
}

#[derive(Serialize)]
struct Name {
    name: String,
}

#[get("/")]
async fn hello() -> Result<impl Responder> {
    let obj = Response {
        message: "Hello World!".to_string(),
    };
    Ok(web::Json(obj))
}

#[get("/{name}")]
async fn get_name(name: web::Path<String>) -> Result<impl Responder> {
    let name_response = Name {
        name: name.to_string(),
    };

    Ok(web::Json(name_response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api/v1")
                .guard(guard::Header("content-type", "application/json"))
                .service(hello)
                .service(get_name),
        )
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}
