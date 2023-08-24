use serde::Deserialize;
use sqlx::FromRow;

pub type Id = i64;

#[derive(Debug, Default, Deserialize, FromRow)]
pub struct LocationType {
    pub id: Id,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Default, Deserialize, FromRow)]
pub struct Location {
    pub id: Id,
    pub name: String,
    pub description: String,
    pub location_type_id: Id,
}

#[derive(Debug, Default, Deserialize, FromRow)]
pub struct Waybill {
    pub id: Id,
    pub consignee: String,
    pub description: String,
    pub routing: String,
    pub via: String,
    pub shipper: String,
    pub from_location_id: Id,
    pub to_location_id: Id,
    pub next_waybill_id: Option<Id>,
}

#[derive(Debug, Default, Deserialize, FromRow)]
pub struct Train {
    pub id: Id,
    pub name: String,
}

#[derive(Debug, Default, Deserialize, FromRow)]
pub struct CarType {
    pub id: Id,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Default, Deserialize, FromRow)]
pub struct Road {
    pub id: Id,
    pub name: String,
    pub shorthand: String,
}

pub struct CarCard {
    pub id: Id,
    pub number: String,
    pub car_type_id: Id,
    pub is_loaded: bool,
    pub notes: String,
    pub waybill_id: Option<Id>,
    pub location_id: Option<Id>,
    pub road_id: Id,
    pub train_id: Option<Id>,
}
