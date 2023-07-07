use crate::{
    api::dto::advert::{AdvertDTO, CreateAdvertDTO, DetailedAdvertDTO},
    domain::{error::CommonError, repository::advert::AdvertQueryParams},
    service::advert::AdvertService,
};
use actix_web::web;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Get {
    id: i32,
}

/// List adverts
#[utoipa::path(
    get,
    path = "/adverts",
    tag = "Adverts",
    responses(
        (status = 200, description = "List of adverts", body = [AdvertDTO]),
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
        (status = 200, description = "Get advert", body = DetailedAdvertDTO),
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
        (status = 200, description = "Advert created", body = DetailedAdvertDTO),
    )
)]
pub async fn create(
    advert_service: web::Data<AdvertService>,
    advert: web::Json<CreateAdvertDTO>,
) -> Result<web::Json<DetailedAdvertDTO>, CommonError> {
    let data = advert_service.create(advert.into_inner().into())?;

    Ok(web::Json(data.into()))
}
