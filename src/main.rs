use actix_web::{web, App, HttpServer};


mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(handlers::welcome_msg))
            .route("/users", web::post().to(handlers::create_user))
            .route("/users/{id}", web::get().to(handlers::get_user_by_id))
            .route("/users/{id}", web::put().to(handlers::update_user_by_id))
            .route("/users/{id}", web::delete().to(handlers::delete_user_by_id))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
