use crate::models::{Location, LocationType};

#[derive(askama::Template)]
#[template(path = "locations/locations.html")]
pub struct LocationsTemplate {
    pub locations: Vec<Location>,
    pub location_types: Vec<LocationType>,
}

#[derive(askama::Template)]
#[template(path = "locations/location_editor.html")]
pub struct LocationEditorTemplate {
    pub location: Location,
    pub location_types: Vec<LocationType>,
}
