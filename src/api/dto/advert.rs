use chrono::NaiveDateTime;
use serde::Serialize;

use crate::domain::model::advert;

#[derive(Debug, Serialize)]
pub struct AdvertDTO {
    pub id: i32,
    pub title: String,
    // TODO: change to type money
    pub price: i64,
    pub created_at: NaiveDateTime,
}

impl From<advert::Advert> for AdvertDTO {
    fn from(advert: advert::Advert) -> Self {
        Self {
            id: advert.id,
            title: advert.title,
            price: advert.price.0,
            created_at: advert.created_at,
        }
    }
}

#[derive(Serialize)]
pub struct DetailedAdvertDTO {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub photo: String,
    pub price: i64,
    pub created_at: NaiveDateTime,
}

impl From<advert::DetailedAdvert> for DetailedAdvertDTO {
    fn from(advert: advert::DetailedAdvert) -> Self {
        Self {
            id: advert.id,
            title: advert.title,
            description: advert.description,
            photo: advert.photo,
            price: advert.price.0,
            created_at: advert.created_at,
        }
    }
}