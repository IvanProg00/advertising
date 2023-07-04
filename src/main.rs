use std::sync::Arc;

use actix_web::{
    middleware::Logger,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use advertising::{
    api,
    infrastructure::{database, repository::postgres::advert::AdvertDieselRepository},
    service::advert::AdvertService,
    setting,
};
use dotenvy::dotenv;
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let s = setting::Setting::from_env().unwrap();
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    let pg_pool = database::postgres::new(s.database_url.clone());
    let advert_repo = Arc::new(AdvertDieselRepository::new(pg_pool));

    HttpServer::new(move || {
        let advert_svc = AdvertService::new(advert_repo.clone());

        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(Data::new(advert_svc))
            .route("/health", web::get().to(HttpResponse::NoContent))
            .service(
                web::scope("/api").service(
                    web::scope("/v1")
                        .service(web::scope("/adverts").configure(api::advert::create_service)),
                ),
            )
    })
    .bind(s.host)?
    .run()
    .await
}
