use crate::domain::{
    error::CommonError,
    model::advert::{Advert, CreateAdvert, DetailedAdvert},
    repository::advert::{AdvertQueryParams, AdvertRepository},
};
use std::sync::Arc;

#[derive(Clone)]
pub struct AdvertService {
    repository: Arc<dyn AdvertRepository>,
}

impl AdvertService {
    pub fn new(repository: Arc<dyn AdvertRepository>) -> Self {
        Self { repository }
    }

    pub fn list(&self, params: AdvertQueryParams) -> Result<Vec<Advert>, CommonError> {
        self.repository.list(params).map_err(|e| e.into())
    }

    pub fn get(&self, id: i32) -> Result<DetailedAdvert, CommonError> {
        self.repository.get(id).map_err(|e| e.into())
    }

    pub fn create(&self, advert: CreateAdvert) -> Result<DetailedAdvert, CommonError> {
        self.repository.create(advert).map_err(|e| e.into())
    }
    // pub fn delete(&self, id: i32) -> RepositoryResult<()>;
    // pub fn update(&self, id: i32, advert: UpdateAdvert) -> RepositoryResult<DetailedAdvert>;
}
