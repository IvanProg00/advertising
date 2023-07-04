use crate::domain::error::CommonError;
use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub message: String,
}

impl actix_web::ResponseError for CommonError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        match self {
            CommonError::NotFound => HttpResponse::NotFound().json(ApiError {
                message: self.to_string(),
            }),
            CommonError::UndefinedError => HttpResponse::InternalServerError().finish(),
        }
    }
}
