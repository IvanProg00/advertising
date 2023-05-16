use crate::{
    domain::model::advert::{Advert, CreateAdvert, DetailedAdvert},
    infrastructure::{
        database::postgres::PostgresPool,
        model::advert::{AdvertDiesel, CreateAdvertDiesel, DetailedAdvertDiesel},
    },
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use std::sync::Arc;

pub struct Postgres {
    pub pool: Arc<PostgresPool>,
}

impl Postgres {
    pub fn new(pg_pool: PostgresPool) -> Self {
        Self {
            pool: Arc::new(pg_pool),
        }
    }

    pub fn list_adverts(&self) -> Vec<Advert> {
        use crate::infrastructure::schema::adverts::dsl::{self, adverts};

        let mut conn = self.pool.clone().get().unwrap();

        let result = adverts
            .select((dsl::id, dsl::title, dsl::price, dsl::created_at))
            .limit(50)
            .load::<AdvertDiesel>(&mut conn)
            .unwrap();

        result.into_iter().map(|v| v.into()).collect()
    }

    pub fn get_advert(&self, id: i32) -> DetailedAdvert {
        use crate::infrastructure::schema::adverts::dsl::{self, adverts};

        let mut conn = self.pool.get().unwrap();

        let result = adverts
            .filter(dsl::id.eq(id))
            .first::<DetailedAdvertDiesel>(&mut conn)
            .unwrap();

        result.into()
    }

    pub fn create_advert(&self, advert: CreateAdvert) {
        use crate::infrastructure::schema::adverts::dsl::adverts;

        let mut conn = self.pool.clone().get().unwrap();
        let advert = CreateAdvertDiesel::from(advert);

        diesel::insert_into(adverts)
            .values(advert)
            .get_result::<DetailedAdvertDiesel>(&mut conn)
            .unwrap();
    }
}
