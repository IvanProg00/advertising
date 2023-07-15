pub mod advert;

use self::advert::AdvertDTO;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[aliases(ListPagingAdverts = ListPaging<AdvertDTO>)]
pub struct ListPaging<T> {
    pub size: i64,
    pub items: Vec<T>,
}
