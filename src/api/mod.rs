use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};

pub mod advert;
pub mod controller;
pub mod dto;
pub mod error;

pub fn create_service(service: &mut ServiceConfig) {
    service.route("/health", web::get().to(HttpResponse::NoContent));
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, test, App};

    #[actix_web::test]
    async fn test_health() {
        let app = test::init_service(App::new().configure(create_service)).await;
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::NO_CONTENT);
    }
}
