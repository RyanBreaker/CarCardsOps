use crate::models::Id;
use sqlx::postgres::PgQueryResult;
use sqlx::{query, query_as, Error, FromRow, PgPool};

#[derive(FromRow, Debug)]
pub struct Location {
    pub id: Id,
    pub name: String,
    pub description: String,
    pub location_type_id: Id,
}

#[allow(unused)]
impl Location {
    pub fn new(name: String, description: String, location_type_id: Id) -> Self {
        Self {
            id: 0,
            name,
            description,
            location_type_id,
        }
    }

    pub async fn insert(&self, pool: &PgPool) -> Result<Location, Error> {
        query_as!(
            Location,
            "INSERT INTO locations (name, description, location_type_id) VALUES ($1, $2, $3) RETURNING *",
            self.name,
            self.description,
            self.location_type_id
        )
            .fetch_one(pool)
            .await
    }

    pub async fn update(&self, pool: &PgPool) -> Result<PgQueryResult, Error> {
        query!(
            "UPDATE locations SET name = $1, description = $2 WHERE id = $3",
            self.name,
            self.description,
            self.id
        )
            .execute(pool)
            .await
    }

    pub async fn select(id: Id, pool: &PgPool) -> Result<Location, Error> {
        query_as!(Location, "SELECT * FROM locations WHERE id = $1", id)
            .fetch_one(pool)
            .await
    }

    pub async fn all(pool: &PgPool) -> Result<Vec<Location>, Error> {
        query_as!(Location, "SELECT * FROM locations")
            .fetch_all(pool)
            .await
    }
}
