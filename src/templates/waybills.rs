use crate::models::{Location, Waybill};

#[derive(askama::Template)]
#[template(path = "waybills/waybills.html")]
pub struct WaybillsTemplate {
    pub waybills: Vec<Waybill>,
}

#[derive(askama::Template)]
#[template(path = "waybills/waybill_editor.html")]
pub struct WaybillEditorTemplate {
    pub waybill: Waybill,
    pub locations: Vec<Location>,
}
