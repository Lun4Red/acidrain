use actix_web::{web, App, HttpServer};

mod handlers;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| async { "Rain REST API" }))
            .service(handlers::index)
            .service(handlers::create)
            .service(handlers::show)
            .service(handlers::update)
            .service(handlers::destroy)
            
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}