pub mod locations;

use crate::models::{Location, LocationType};

#[derive(askama::Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub locations: Vec<Location>,
    pub location_types: Vec<LocationType>,
}
