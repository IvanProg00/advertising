use chrono::NaiveDateTime;
use diesel::data_types::Cents;

#[derive(Debug)]
pub struct Advert {
    pub id: i32,
    pub title: String,
    // TODO: Change to iso_currency::Currency
    pub price: Cents,
    pub created_at: NaiveDateTime,
}

#[derive(Debug)]
pub struct DetailedAdvert {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub photo: String,
    // TODO: Change to iso_currency::Currency
    pub price: Cents,
    pub created_at: NaiveDateTime,
}

pub struct CreateAdvert {
    pub title: String,
    pub description: String,
    pub photo: String,
    // TODO: Change to iso_currency::Currency
    pub price: Cents,
}

pub struct UpdateAdvert {
    pub title: String,
    pub description: String,
    pub photo: String,
    // TODO: Change to iso_currency::Currency
    pub price: Cents,
}
