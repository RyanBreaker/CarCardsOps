mod location;
mod location_type;
mod waybill;
mod road;

use crate::handlers::location::*;
use crate::handlers::location_type::*;
use crate::handlers::waybill::{
    waybill_editor, waybill_new, waybill_post, waybill_put, waybills_view,
};
use poem::{get, post, Route};
use crate::handlers::road::{road_editor, road_new, road_post, road_put, roads_view};

pub fn location_types() -> Route {
    Route::new()
        .at("/", get(location_types_view))
        .at("/new", get(location_type_new))
        .at("/:id", post(location_type_post).put(location_type_update))
        .at("/:id/edit", get(location_type_editor))
}

pub fn locations() -> Route {
    Route::new()
        .at("/", get(locations_view))
        .at("/new", get(location_new))
        .at("/:id", post(location_post).put(location_put))
        .at("/:id/edit", get(location_editor))
}

pub fn waybills() -> Route {
    Route::new()
        .at("/", get(waybills_view))
        .at("/new", get(waybill_new))
        .at("/:id", post(waybill_post).put(waybill_put))
        .at("/:id/edit", get(waybill_editor))
}

pub fn roads() -> Route {
    Route::new()
        .at("/", get(roads_view))
        .at("/new", get(road_new))
        .at("/:id", post(road_post).put(road_put))
        .at("/:id/edit", get(road_editor))
}
