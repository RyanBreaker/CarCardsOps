use serde::Deserialize;
use sqlx::FromRow;

pub type Id = i64;

#[derive(FromRow, Debug, Deserialize, Default)]
pub struct LocationType {
    pub id: Id,
    pub name: String,
    pub description: String,
}

#[derive(FromRow, Debug, Clone, Deserialize, Default)]
pub struct Location {
    pub id: Id,
    pub name: String,
    pub description: String,
    pub location_type_id: Id,
}

#[derive(Debug, FromRow, Deserialize, Default)]
pub struct Waybill {
    pub id: Id,
    pub name: String,
    pub description: String,
    pub routing: String,
    pub from_location_id: Id,
    pub to_location_id: Id,
    pub next_waybill_id: Option<Id>,
    pub prev_waybill_id: Option<Id>,
}
