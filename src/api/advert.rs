use super::controller::advert;
use actix_web::web;

pub fn create_service(service: &mut web::ServiceConfig) {
    service
        .route("", web::get().to(advert::list))
        .route("/{id}", web::get().to(advert::get))
        .route("/{id}", web::delete().to(advert::delete))
        .route("", web::post().to(advert::create));
}
