use crate::models::{Id, Location, LocationType};
use crate::templates::locations::{LocationEditorTemplate, LocationTemplate, LocationsTemplate};
use askama::Template;
use poem::http::StatusCode;
use poem::web::{Data, Form, Html, Path};
use poem::{handler, IntoResponse};
use sqlx::PgPool;

#[handler]
pub async fn location_view(Path(id): Path<Id>, Data(pool): Data<&PgPool>) -> Html<String> {
    let location = Location::select(id, pool).await.unwrap();
    Html(LocationTemplate { location }.render().unwrap())
}

#[handler]
pub async fn locations_view(Data(pool): Data<&PgPool>) -> Html<String> {
    let (locations, location_types) =
        match tokio::try_join!(Location::all(pool), LocationType::all(pool)) {
            Ok(a) => a,
            Err(e) => panic!("{e}"),
        };
    Html(
        LocationsTemplate {
            locations,
            location_types,
        }
        .render()
        .unwrap(),
    )
}

#[handler]
pub async fn location_insert(
    Form(location): Form<Location>,
    Data(pool): Data<&PgPool>,
) -> impl IntoResponse {
    location.insert(pool).await.unwrap();
    StatusCode::OK.with_header("HX-Refresh", "true")
}

#[handler]
pub async fn location_update(
    Path(_id): Path<Id>,
    Form(location): Form<Location>,
    Data(pool): Data<&PgPool>,
) -> impl IntoResponse {
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
