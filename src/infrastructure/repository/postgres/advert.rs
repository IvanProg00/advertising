use crate::{
    domain::{
        error::{RepositoryError, RepositoryResult},
        model::advert::{Advert, CreateAdvert, DetailedAdvert, UpdateAdvert},
        repository::advert::{AdvertQueryParams, AdvertRepository},
    },
    infrastructure::{
        database::postgres::PostgresPool,
        model::advert::{AdvertDiesel, CreateAdvertDiesel, DetailedAdvertDiesel},
    },
};
use diesel::data_types::Cents;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use std::sync::Arc;

pub struct AdvertDieselRepository {
    pub pool: Arc<PostgresPool>,
}

impl AdvertDieselRepository {
    pub fn new(pg_pool: PostgresPool) -> Self {
        Self {
            pool: Arc::new(pg_pool),
        }
    }
}

impl AdvertRepository for AdvertDieselRepository {
    fn list(&self, params: AdvertQueryParams) -> RepositoryResult<Vec<Advert>> {
        use crate::infrastructure::schema::adverts::dsl::{self, adverts};

        let mut conn = self.pool.clone().get().unwrap();

        let data = adverts
            .select((dsl::id, dsl::title, dsl::price, dsl::created_at))
            .limit(params.size())
            .offset(params.offset())
            .load::<AdvertDiesel>(&mut conn)
            .map_err(RepositoryError::from)?;

        Ok(data.into_iter().map(|v| v.into()).collect())
    }

    fn count(&self) -> RepositoryResult<i64> {
        use crate::infrastructure::schema::adverts::dsl::adverts;

        let mut conn = self.pool.clone().get().unwrap();
        let count = adverts.count().get_result(&mut conn)?;

        Ok(count)
    }

    fn get(&self, id: i32) -> RepositoryResult<DetailedAdvert> {
        use crate::infrastructure::schema::adverts::dsl::{self, adverts};

        let mut conn = self.pool.get().unwrap();

        let result = adverts
            .filter(dsl::id.eq(id))
            .first::<DetailedAdvertDiesel>(&mut conn)?;

        Ok(result.into())
    }

    fn create(&self, advert: CreateAdvert) -> RepositoryResult<DetailedAdvert> {
        use crate::infrastructure::schema::adverts::dsl::adverts;

        let mut conn = self.pool.clone().get().unwrap();
        let advert = CreateAdvertDiesel::from(advert);

        let result = diesel::insert_into(adverts)
            .values(advert)
            .get_result::<DetailedAdvertDiesel>(&mut conn)?
            .into();

        Ok(result)
    }

    fn delete(&self, id: i32) -> RepositoryResult<()> {
        use crate::infrastructure::schema::adverts::dsl::{self, adverts};

        let mut conn = self.pool.get().unwrap();

        diesel::delete(adverts)
            .filter(dsl::id.eq(id))
            .execute(&mut conn)?;

        Ok(())
    }

    fn update(&self, id: i32, advert: UpdateAdvert) -> RepositoryResult<DetailedAdvert> {
        use crate::infrastructure::schema::adverts::dsl::{self, adverts};

        let mut conn = self.pool.get().unwrap();

        let result = diesel::update(adverts.filter(dsl::id.eq(id)))
            .set((
                dsl::title.eq(advert.title),
                dsl::description.eq(advert.description),
                dsl::photo.eq(advert.photo),
                dsl::price.eq(Cents(advert.price)),
            ))
            .get_result::<DetailedAdvertDiesel>(&mut conn)?
            .into();

        Ok(result)
    }
}
