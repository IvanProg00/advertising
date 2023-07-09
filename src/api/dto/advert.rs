use crate::domain::model::advert;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct AdvertDTO {
    pub id: i32,
    pub title: String,
    pub price: i64,
    pub created_at: NaiveDateTime,
}

impl From<advert::Advert> for AdvertDTO {
    fn from(advert: advert::Advert) -> Self {
        Self {
            id: advert.id,
            title: advert.title,
            price: advert.price,
            created_at: advert.created_at,
        }
    }
}

#[derive(Serialize, ToSchema)]
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
            price: advert.price,
            created_at: advert.created_at,
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct CreateAdvertDTO {
    pub title: String,
    pub description: String,
    pub photo: String,
    pub price: i64,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateAdvertDTO {
    pub title: String,
    pub description: String,
    pub photo: String,
    pub price: i64,
}
