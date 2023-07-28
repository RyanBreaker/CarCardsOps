mod location;
mod location_type;

use crate::handlers::location::*;
use crate::handlers::location_type::*;
use poem::{get, Route};

pub fn location_types() -> Route {
    Route::new()
        .at("/", get(location_types_view))
        .at("/:id", get(location_type_view).put(location_type_update))
        .at("/:id/edit", get(location_type_editor))
}

pub fn locations() -> Route {
    Route::new()
        .at("/", get(locations_view).post(location_insert))
        .at("/:id", get(location_view).put(location_update))
        .at("/:id/edit", get(location_edit))
}
