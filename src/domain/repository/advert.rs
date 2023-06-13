use crate::domain::{
    error::RepositoryResult,
    model::advert::{Advert, CreateAdvert, DetailedAdvert, UpdateAdvert},
};

pub trait AdvertRepository {
    fn list(&self) -> RepositoryResult<Vec<Advert>>;
    fn get(&self, id: i32) -> RepositoryResult<DetailedAdvert>;
    fn create(&self, advert: CreateAdvert) -> RepositoryResult<DetailedAdvert>;
    fn delete(&self, id: i32) -> RepositoryResult<()>;
    fn update(&self, id: i32, advert: UpdateAdvert) -> RepositoryResult<DetailedAdvert>;
}
