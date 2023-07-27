use crate::models::Id;
use sqlx::{query_as, Error, FromRow, PgPool, query};
use sqlx::postgres::PgQueryResult;

#[derive(Debug, FromRow)]
pub struct Waybill {
    pub id: Id,
    pub description: String,
    pub routing: String,
    pub from_location_id: Id,
    pub to_location_id: Id,
}

#[allow(unused)]
impl Waybill {
    pub fn new(
        description: String,
        routing: String,
        from_location_id: Id,
        to_location_id: Id,
    ) -> Self {
        Self {
            id: 0,
            description,
            routing,
            from_location_id,
            to_location_id,
        }
    }

    pub async fn insert(&self, pool: &PgPool) -> Result<Self, Error> {
        query_as!(
            Waybill,
            "INSERT INTO waybills (description, routing, from_location_id, to_location_id) VALUES ($1, $2, $3, $4) RETURNING *",
            self.description,
            self.routing,
            self.from_location_id,
            self.to_location_id
        )
            .fetch_one(pool)
            .await
    }

    pub async fn update(&self, pool: &PgPool) -> Result<Self, Error> {
        query_as!(
            "UPDATE waybills SET description = $1, routing = $2, from_location_id = $3, to_location_id = $4 WHERE id = $5 RETURNING *",
            self.description,
            self.routing,
            self.from_location_id,
            self.to_location_id,
            self.id
        )
            .fetch_one(pool)
            .await
    }

    pub async fn select(id: Id, pool: &PgPool) -> Result<Self, Error> {
        query_as!(
            Waybill,
            "SELECT * FROM waybills WHERE id = $1",
            id
        )
            .fetch_one(pool)
            .await
    }

    pub async fn all(pool: &PgPool) -> Result<Vec<Self>, Error> {
        query_as!(
            Waybill,
            "SELECT * FROM waybills"
        )
            .fetch_all(pool)
            .await
    }
}
