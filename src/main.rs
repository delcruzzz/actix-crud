#[macro_use]
extern crate diesel;

/*
    conexión con postgreSQL

    después de crear las migraciones de diesel y generamos 'src/schema.rs'. Luego 
    se agrega el código para conectarse a PostgreSQL, y usamos [R2D2] para agrupar 
    conexiones
*/

use actix_web::{get, web, App, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

// se define un custom type para la conexión pool
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/")]
async fn welcome() -> impl Responder {
    format!("Actix and PostgreSQL")
}

mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // cargar la variable de entorno
    dotenv::dotenv().ok();

    // configurcación del pool para la conexión de la base de datos
    let database_url = std::env::var("DATABASE_URL").expect("error reading env var");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("failed to created pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
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
