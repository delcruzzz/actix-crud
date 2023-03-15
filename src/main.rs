use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn welcome() -> impl Responder {
    format!("Actix and PostgreSQL")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(welcome)
    })
    .bind(("127.0.0.1", 4000))?
    .run().await
}
