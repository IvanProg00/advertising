use crate::api::{controller, dto};
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
            dto::advert::AdvertDTO, dto::advert::DetailedAdvertDTO, dto::advert::CreateAdvertDTO,
            dto::advert::UpdateAdvertDTO,
        ),
    ),
    tags(
        (name = "Adverts", description = "Adverts endpoints."),
    )
)]
pub struct ApiDoc;
