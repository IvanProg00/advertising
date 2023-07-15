use crate::{
    api::dto::{
        advert::{AdvertDTO, CreateAdvertDTO, DetailedAdvertDTO, UpdateAdvertDTO},
        ListPaging,
    },
    domain::{error::CommonError, repository::advert::AdvertQueryParams},
    service::advert::AdvertService,
};
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, IntoParams)]
pub struct IdParam {
    /// Advert id
    id: i32,
}

/// List adverts
#[utoipa::path(
    get,
    path = "/adverts",
    tag = "Adverts",
    params(
        AdvertQueryParams
    ),
    responses(
        (status = StatusCode::OK, description = "List of adverts", body = ListPagingAdverts),
    )
)]
pub async fn list(
    advert_service: web::Data<AdvertService>,
    params: web::Query<AdvertQueryParams>,
) -> Result<web::Json<ListPaging<AdvertDTO>>, CommonError> {
    let data = advert_service
        .list(params.into_inner())?
        .into_iter()
        .map(|val| val.into())
        .collect();

    let count = advert_service.count()?;

    Ok(web::Json(ListPaging {
        size: count,
        items: data,
    }))
}

/// Get advert by id
#[utoipa::path(
    get,
    path = "/adverts/{id}",
    tag = "Adverts",
    responses(
        (status = StatusCode::OK, description = "Get advert", body = DetailedAdvertDTO),
    ),
    params(
        IdParam
    )
)]
pub async fn get(
    advert_service: web::Data<AdvertService>,
    path: web::Path<IdParam>,
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
        IdParam,
    )
)]
pub async fn delete(
    advert_service: web::Data<AdvertService>,
    path: web::Path<IdParam>,
) -> Result<impl Responder, CommonError> {
    advert_service.delete(path.id)?;

    Ok(HttpResponse::NoContent().finish())
}

/// Update advert by id
#[utoipa::path(
    put,
    path = "/adverts/{id}",
    tag = "Adverts",
    request_body = UpdateAdvertDTO,
    responses(
        (status = StatusCode::OK, description = "Advert updated", body = DetailedAdvertDTO),
    ),
    params(
        IdParam,
    )
)]
pub async fn update(
    advert_service: web::Data<AdvertService>,
    path: web::Path<IdParam>,
    advert: web::Json<UpdateAdvertDTO>,
) -> Result<web::Json<DetailedAdvertDTO>, CommonError> {
    let data = advert_service.update(path.id, advert.into_inner().into())?;

    Ok(web::Json(data.into()))
}
