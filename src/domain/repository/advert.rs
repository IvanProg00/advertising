use crate::domain::{
    error::RepositoryResult,
    model::advert::{Advert, CreateAdvert, DetailedAdvert, UpdateAdvert},
};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, ToSchema, Default)]
#[serde(rename_all = "snake_case")]
pub enum AdvertSortBy {
    CreatedAtAsc,
    #[default]
    CreatedAtDesc,
    PriceAsc,
    PriceDesc,
}

#[derive(Deserialize, IntoParams)]
pub struct AdvertQueryParams {
    size: Option<i64>,
    offset: Option<i64>,
    pub sort_by: AdvertSortBy,
}

pub trait AdvertRepository {
    fn list(&self, params: AdvertQueryParams) -> RepositoryResult<Vec<Advert>>;
    fn count(&self) -> RepositoryResult<i64>;
    fn get(&self, id: i32) -> RepositoryResult<DetailedAdvert>;
    fn create(&self, advert: CreateAdvert) -> RepositoryResult<DetailedAdvert>;
    fn delete(&self, id: i32) -> RepositoryResult<()>;
    fn update(&self, id: i32, advert: UpdateAdvert) -> RepositoryResult<DetailedAdvert>;
}

impl AdvertQueryParams {
    pub fn size(&self) -> i64 {
        self.size.unwrap_or(super::DEFAULT_SIZE)
    }

    pub fn offset(&self) -> i64 {
        self.offset.unwrap_or(super::DEFAULT_OFFSET)
    }
}
