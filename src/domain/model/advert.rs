use crate::api::dto::advert::{CreateAdvertDTO, UpdateAdvertDTO};
use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct Advert {
    pub id: i32,
    pub title: String,
    pub price: i64,
    pub created_at: NaiveDateTime,
}

#[derive(Debug)]
pub struct DetailedAdvert {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub photo: String,
    pub price: i64,
    pub created_at: NaiveDateTime,
}

pub struct CreateAdvert {
    pub title: String,
    pub description: String,
    pub photo: String,
    pub price: i64,
}

pub struct UpdateAdvert {
    pub title: String,
    pub description: String,
    pub photo: String,
    pub price: i64,
}

impl From<CreateAdvertDTO> for CreateAdvert {
    fn from(advert: CreateAdvertDTO) -> Self {
        Self {
            title: advert.title,
            description: advert.description,
            photo: advert.photo,
            price: advert.price,
        }
    }
}

impl From<UpdateAdvertDTO> for UpdateAdvert {
    fn from(advert: UpdateAdvertDTO) -> Self {
        Self {
            title: advert.title,
            description: advert.description,
            photo: advert.photo,
            price: advert.price,
        }
    }
}
