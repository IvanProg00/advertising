use crate::{
    api::{controller, dto},
    domain::repository::advert::AdvertSortBy,
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    servers(
        (url = "http://localhost:8080/api/v1", description = "API v1"),
    ),
    paths(
        controller::advert::list, controller::advert::get, controller::advert::create, controller::advert::delete,
        controller::advert::update,
    ),
    components(
        schemas(
            dto::ListPagingAdverts, dto::advert::AdvertDTO, dto::advert::DetailedAdvertDTO,
            dto::advert::CreateAdvertDTO, dto::advert::UpdateAdvertDTO, AdvertSortBy,
        ),
    ),
    tags(
        (name = "Adverts", description = "Adverts endpoints."),
    )
)]
pub struct ApiDoc;
