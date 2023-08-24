use crate::models::Road;
use crate::templates::filters;

#[derive(askama::Template)]
#[template(path = "roads/roads.html")]
pub struct RoadsTemplate {
    pub roads: Vec<Road>
}

#[derive(askama::Template)]
#[template(path = "roads/road_editor.html")]
pub struct RoadEditorTemplate {
    pub road: Road,
    pub is_new: bool,
}