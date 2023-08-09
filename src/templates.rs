pub mod location_types;
pub mod locations;
pub mod waybills;

use crate::models::{Location, LocationType};

#[derive(askama::Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub locations: Vec<Location>,
    pub location_types: Vec<LocationType>,
}

mod filters {
    pub fn yesno<'a>(value: &bool, a: &'a str, b: &'a str) -> askama::Result<&'a str> {
        if *value {
            Ok(a)
        } else {
            Ok(b)
        }
    }
}
