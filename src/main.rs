use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/health", web::get().to(HttpResponse::NoContent)))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
