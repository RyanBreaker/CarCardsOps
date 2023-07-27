use askama::{Template};
use poem::handler;
use poem::web::{Data, Form, Html, Path};
use sqlx::PgPool;
use crate::models::{Id, Location, LocationType};
use crate::templates::locations::{LocationEditorTemplate, LocationsTemplate, LocationTemplate};

#[handler]
pub async fn location_view(Path(id): Path<Id>, Data(pool): Data<&PgPool>) -> Html<String> {
    let location = Location::select(id, pool).await.unwrap();
    Html(LocationTemplate { location }.render().unwrap())
}

#[handler]
pub async fn locations_view(Data(pool): Data<&PgPool>) -> Html<String> {
    let locations = Location::all(pool).await.unwrap();
    Html(LocationsTemplate { locations }.render().unwrap())
}

#[handler]
pub async fn location_update(
    Path(_id): Path<Id>,
    Form(location): Form<Location>,
    Data(pool): Data<&PgPool>,
) -> Html<String> {
    let location = location.update(pool).await.unwrap();
    Html(LocationTemplate { location }.render().unwrap())
}

#[handler]
pub async fn location_edit(Path(id): Path<Id>, Data(pool): Data<&PgPool>) -> Html<String> {
    let template = LocationEditorTemplate {
        location: Location::select(id, pool).await.unwrap(),
        location_types: LocationType::all(pool).await.unwrap(),
    };
    Html(template.render().unwrap())
}

#[handler]
pub async fn location_creator() -> Html<String> {
    Html("".into())
}
