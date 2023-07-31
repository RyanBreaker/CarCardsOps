use serde::Deserialize;
use sqlx::FromRow;

pub type Id = i32;

#[derive(FromRow, Debug, Deserialize)]
pub struct LocationType {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(FromRow, Debug, Clone, Deserialize)]
pub struct Location {
    pub id: Id,
    pub name: String,
    pub description: String,
    pub location_type_id: Id,
}

#[derive(Debug, FromRow, Deserialize)]
pub struct Waybill {
    pub id: Id,
    pub description: String,
    pub routing: String,
    pub from_location_id: Id,
    pub to_location_id: Id,
}
