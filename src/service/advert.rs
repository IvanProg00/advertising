use std::sync::Arc;

use crate::domain::{
    error::CommonError,
    model::advert::{Advert, DetailedAdvert},
    repository::advert::{AdvertQueryParams, AdvertRepository},
};

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

    // fn create(&self, advert: CreateAdvert) -> RepositoryResult<DetailedAdvert>;
    // fn delete(&self, id: i32) -> RepositoryResult<()>;
    // fn update(&self, id: i32, advert: UpdateAdvert) -> RepositoryResult<DetailedAdvert>;
}
