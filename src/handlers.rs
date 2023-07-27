mod location;
mod location_type;

use poem::{get, Route};
use crate::handlers::location::{location_edit, location_update, location_view, locations_view};
use crate::handlers::location_type::{location_type_editor, location_type_update, location_type_view, location_types_view};

pub fn location_types() -> Route {
    Route::new()
        .at("/", get(location_types_view))
        .at("/:id", get(location_type_view).put(location_type_update))
        .at("/:id/edit", get(location_type_editor))
}

pub fn locations() -> Route {
    Route::new()
        .at("/", get(locations_view))
        .at("/:id", get(location_view).put(location_update))
        .at("/:id/edit", get(location_edit))
}
