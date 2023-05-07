use actix_web::{web, App, HttpResponse, HttpServer};
use advertising::setting;
use dotenvy::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let s = setting::Setting::from_env().unwrap();

    HttpServer::new(|| App::new().route("/health", web::get().to(HttpResponse::NoContent)))
        .bind(s.host)?
        .run()
        .await
}
