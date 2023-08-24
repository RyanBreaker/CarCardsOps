use crate::models::{Location, Waybill};
use crate::templates::filters;

pub struct WaybillLocation {
    pub waybill: Waybill,
    pub from_location: Location,
    pub to_location: Location,
}

#[derive(askama::Template)]
#[template(path = "waybills/waybills.html")]
pub struct WaybillsTemplate {
    pub waybills: Vec<WaybillLocation>,
}

#[derive(askama::Template)]
#[template(path = "waybills/waybill_editor.html")]
pub struct WaybillEditorTemplate {
    pub waybill: Waybill,
    pub waybills: Vec<Waybill>,
    pub locations: Vec<Location>,
    pub is_new: bool,
}
