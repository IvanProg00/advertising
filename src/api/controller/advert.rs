use crate::{
    api::dto::advert::{AdvertDTO, CreateAdvertDTO, DetailedAdvertDTO},
    domain::{error::CommonError, repository::advert::AdvertQueryParams},
    service::advert::AdvertService,
};
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Get {
    id: i32,
}

#[derive(Deserialize)]
pub struct Delete {
    id: i32,
}

/// List adverts
#[utoipa::path(
    get,
    path = "/adverts",
    tag = "Adverts",
    responses(
        (status = StatusCode::OK, description = "List of adverts", body = [AdvertDTO]),
    )
)]
pub async fn list(
    advert_service: web::Data<AdvertService>,
) -> Result<web::Json<Vec<AdvertDTO>>, CommonError> {
    let data = advert_service.list(AdvertQueryParams {
        offset: 0,
        limit: 5,
    })?;

    let resp = data.into_iter().map(|val| val.into()).collect();

    Ok(web::Json(resp))
}

/// Get advert by id
#[utoipa::path(
    get,
    path = "/adverts/{id}",
    tag = "Adverts",
    responses(
        (status = StatusCode::OK, description = "Get advert", body = DetailedAdvertDTO),
    )
)]
pub async fn get(
    advert_service: web::Data<AdvertService>,
    path: web::Path<Get>,
) -> Result<web::Json<DetailedAdvertDTO>, CommonError> {
    let data = advert_service.get(path.id)?;

    Ok(web::Json(data.into()))
}

/// Create advert
#[utoipa::path(
    post,
    path = "/adverts",
    tag = "Adverts",
    request_body = CreateAdvertDTO,
    responses(
        (status = StatusCode::CREATED, description = "Advert created", body = DetailedAdvertDTO),
    )
)]
pub async fn create(
    advert_service: web::Data<AdvertService>,
    advert: web::Json<CreateAdvertDTO>,
) -> Result<impl Responder, CommonError> {
    let data: DetailedAdvertDTO = advert_service.create(advert.into_inner().into())?.into();

    Ok(HttpResponse::Created().json(data))
}

/// Delete advert by id
#[utoipa::path(
    delete,
    path = "/adverts/{id}",
    tag = "Adverts",
    responses(
        (status = StatusCode::NO_CONTENT, description = "Advert deleted"),
    ),
    params(
        ("id" = String, Path, description = "Advert id"),
    )
)]
pub async fn delete(
    advert_service: web::Data<AdvertService>,
    path: web::Path<Delete>,
) -> Result<impl Responder, CommonError> {
    advert_service.delete(path.id)?;

    Ok(HttpResponse::NoContent().finish())
}
