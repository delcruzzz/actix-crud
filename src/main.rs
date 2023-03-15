use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn welcome() -> impl Responder {
    format!("Actix and PostgreSQL")
}

mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(welcome)
            .service(handlers::tweets::get_tweets)
            .service(handlers::tweets::get_tweet_by_id)
            .service(handlers::tweets::create_tweet)
            .service(handlers::tweets::update_tweet)
            .service(handlers::tweets::destroy_tweet)
    })
    .bind(("127.0.0.1", 4000))?
    .run().await
}
