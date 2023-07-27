use crate::models::{Location, LocationType};

#[derive(askama::Template)]
#[template(path = "locations.html")]
pub struct LocationsTemplate {
    pub locations: Vec<Location>,
}

#[derive(askama::Template)]
#[template(path = "partials/location/location.html")]
pub struct LocationTemplate {
    pub location: Location,
}

#[derive(askama::Template)]
#[template(path = "partials/location/location_editor.html")]
pub struct LocationEditorTemplate {
    pub location: Location,
    pub location_types: Vec<LocationType>,
}
