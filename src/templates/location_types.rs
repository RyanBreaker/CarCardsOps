use crate::models::LocationType;
use crate::templates::filters;

#[derive(askama::Template)]
#[template(path = "location_types/location_types.html")]
pub struct LocationTypesTemplate {
    pub location_types: Vec<LocationType>,
}

#[derive(askama::Template)]
#[template(path = "location_types/location_type_editor.html")]
pub struct LocationTypeEditorTemplate {
    pub location_type: LocationType,
    pub is_new: bool,
}
