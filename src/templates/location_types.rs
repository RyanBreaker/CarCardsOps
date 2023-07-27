use crate::models::LocationType;

#[derive(askama::Template)]
#[template(path = "location_types.html")]
pub struct LocationTypesTemplate {
    pub location_types: Vec<LocationType>,
}

#[derive(askama::Template)]
#[template(path = "partials/location_type/location_type.html")]
pub struct LocationTypeTemplate {
    pub location_type: LocationType,
}

#[derive(askama::Template)]
#[template(path = "partials/location_type/location_type_editor.html")]
pub struct LocationTypeEditorTemplate {
    pub location_type: LocationType
}
