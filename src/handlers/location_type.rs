use crate::models::{Id, LocationType};
use crate::templates::location_types::*;
use askama::Template;
use poem::web::{Data, Form, Html, Path, Redirect};
use poem::{handler, IntoResponse};
use sqlx::{query, query_as, SqlitePool};

#[handler]
pub async fn location_types_view(Data(pool): Data<&SqlitePool>) -> impl IntoResponse {
    let location_types = query_as!(LocationType, "SELECT * FROM location_types")
        .fetch_all(pool)
        .await
        .unwrap();
    Html(LocationTypesTemplate { location_types }.render().unwrap())
}

#[handler]
pub async fn location_type_new() -> impl IntoResponse {
    Html(
        LocationTypeEditorTemplate {
            location_type: LocationType::default(),
            is_new: true,
        }
        .render()
        .unwrap(),
    )
}

#[handler]
pub async fn location_type_post(
    Path(id): Path<Id>,
    Form(location_type): Form<LocationType>,
    Data(pool): Data<&SqlitePool>,
) -> impl IntoResponse {
    query!(
        "UPDATE location_types SET name = $1, description = $2 WHERE id = $3",
        location_type.name,
        location_type.description,
        id
    )
    .execute(pool)
    .await
    .unwrap();
    Redirect::see_other("/location_types")
}

#[handler]
pub async fn location_type_update(
    Path(id): Path<Id>,
    Form(location_type): Form<LocationType>,
    Data(pool): Data<&SqlitePool>,
) -> impl IntoResponse {
    query!(
        "UPDATE location_types SET name = $1, description = $2 WHERE id = $3",
        location_type.name,
        location_type.description,
        id
    )
    .fetch_one(pool)
    .await
    .unwrap();
    Redirect::see_other("/location_types")
}

#[handler]
pub async fn location_type_editor(
    Path(id): Path<Id>,
    Data(pool): Data<&SqlitePool>,
) -> impl IntoResponse {
    let location_type = query_as!(
        LocationType,
        "SELECT * FROM location_types WHERE id = $1",
        id
    )
    .fetch_one(pool)
    .await
    .unwrap();
    Html(
        LocationTypeEditorTemplate { location_type, is_new: false }
            .render()
            .unwrap(),
    )
}
