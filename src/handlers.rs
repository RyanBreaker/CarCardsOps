use crate::models::{Id, Location, LocationType};
use crate::templates::locations::{LocationEditorTemplate, LocationTemplate, LocationsTemplate};
use askama::Template;
use poem::web::{Data, Form, Html, Path};
use poem::{get, handler, Route};
use sqlx::PgPool;

pub fn locations() -> Route {
    Route::new()
        .at("/", get(locations_view))
        .at("/:id", get(location_view).put(location_update))
        .at("/:id/edit", get(location_editor))
}

#[handler]
async fn location_view(Path(id): Path<Id>, Data(pool): Data<&PgPool>) -> Html<String> {
    let location = Location::select(id, pool).await.unwrap();
    Html(LocationTemplate { location }.render().unwrap())
}

#[handler]
async fn locations_view(Data(pool): Data<&PgPool>) -> Html<String> {
    let locations = Location::all(pool).await.unwrap();
    Html(LocationsTemplate { locations }.render().unwrap())
}

#[handler]
async fn location_update(
    Path(_id): Path<Id>,
    Form(location): Form<Location>,
    Data(pool): Data<&PgPool>,
) -> Html<String> {
    location.update(pool).await.unwrap();
    Html(LocationTemplate { location }.render().unwrap())
}

#[handler]
async fn location_editor(Path(id): Path<Id>, Data(pool): Data<&PgPool>) -> Html<String> {
    let template = LocationEditorTemplate {
        location: Location::select(id, pool).await.unwrap(),
        location_types: LocationType::all(pool).await.unwrap(),
    };
    Html(template.render().unwrap())
}

#[handler]
async fn location_creator() -> Html<String> {
    Html("".into())
}
