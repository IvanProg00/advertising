use actix_web::{
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use advertising::{
    api,
    infrastructure::{database, repository::postgres::advert::AdvertDieselRepository},
    openapi::ApiDoc,
    service::advert::AdvertService,
    setting,
};
use dotenvy::dotenv;
use env_logger::Env;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let s = setting::Setting::from_env().unwrap();
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    let pg_pool = database::postgres::new(s.database_url.clone());
    let advert_repo = Arc::new(AdvertDieselRepository::new(pg_pool));

    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        let advert_svc = AdvertService::new(advert_repo.clone());

        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(Data::new(advert_svc))
            .configure(api::create_service)
            .service(
                web::scope("/api").service(
                    web::scope("/v1")
                        .service(web::scope("/adverts").configure(api::advert::create_service)),
                ),
            )
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
    })
    .bind(s.host)?
    .run()
    .await
}
