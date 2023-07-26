use sqlx::{query_as, Error, PgPool};

#[derive(sqlx::FromRow, Debug)]
pub struct LocationType {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[allow(unused)]
impl LocationType {
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: 0,
            name,
            description,
        }
    }

    pub async fn insert(&self, pool: &PgPool) -> Result<LocationType, Error> {
        query_as!(
            LocationType,
            "INSERT INTO location_types (name, description) VALUES ($1, $2) RETURNING *;",
            self.name,
            self.description
        )
        .fetch_one(pool)
        .await
    }

    pub async fn select(id: i32, pool: &PgPool) -> Result<LocationType, Error> {
        query_as!(
            LocationType,
            "SELECT * FROM location_types WHERE id = $1;",
            id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn all(pool: &PgPool) -> Result<Vec<LocationType>, Error> {
        query_as!(LocationType, "SELECT * FROM location_types;")
            .fetch_all(pool)
            .await
    }
}
