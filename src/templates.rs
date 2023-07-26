use crate::models::{Location, LocationType};

#[derive(askama::Template)]
#[template(path = "index.html")]
pub struct HomeTemplate {
    pub location_types: Vec<LocationType>,
    pub locations: Vec<Location>,
}
