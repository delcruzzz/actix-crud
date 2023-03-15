use actix_web::{get, post, put, delete, web, HttpResponse, Responder};

#[get("/tweets")]
pub async fn get_tweets() -> impl Responder {
  HttpResponse::Ok().body("Tweet#index")
}

#[get("/tweets/{id}")]
pub async fn get_tweet_by_id(id: web::Path<String>) -> impl Responder {
  HttpResponse::Ok().body(format!("Tweet#show {}", id))
}

#[post("/tweets")]
pub async fn create_tweet() -> impl Responder {
  HttpResponse::Ok().body("Tweet#new")
}

#[put("/tweets/{id}")]
pub async fn update_tweet(id: web::Path<String>) -> impl Responder {
  HttpResponse::Ok().body(format!("Tweet#edit {}", id))
}

#[delete("/tweets/{id}")]
pub async fn destroy_tweet(id: web::Path<String>) -> impl Responder {
HttpResponse::Ok().body(format!("Tweet#delete {}", id))
}