use crate::{
    domain::model::advert::{Advert, CreateAdvert, DetailedAdvert},
    infrastructure::schema::adverts,
};
use chrono::NaiveDateTime;
use diesel::{data_types::Cents, Insertable, Queryable};

#[derive(Queryable)]
pub struct AdvertDiesel {
    pub id: i32,
    pub title: String,
    pub price: Cents,
    pub created_at: NaiveDateTime,
}

impl From<AdvertDiesel> for Advert {
    fn from(a: AdvertDiesel) -> Self {
        Self {
            id: a.id,
            title: a.title,
            price: a.price.0,
            created_at: a.created_at,
        }
    }
}

#[derive(Queryable, Debug)]
pub struct DetailedAdvertDiesel {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub photo: String,
    pub price: Cents,
    pub created_at: NaiveDateTime,
}

impl From<DetailedAdvertDiesel> for DetailedAdvert {
    fn from(a: DetailedAdvertDiesel) -> Self {
        Self {
            id: a.id,
            title: a.title,
            description: a.description,
            photo: a.photo,
            price: a.price.0,
            created_at: a.created_at,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = adverts)]
pub struct CreateAdvertDiesel {
    pub title: String,
    pub description: String,
    pub photo: String,
    pub price: Cents,
}

impl From<CreateAdvert> for CreateAdvertDiesel {
    fn from(a: CreateAdvert) -> Self {
        Self {
            title: a.title,
            description: a.description,
            photo: a.photo,
            price: Cents(a.price),
        }
    }
}
