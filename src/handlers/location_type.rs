use crate::models::{Id, LocationType};
use crate::templates::location_types::*;
use askama::Template;
use poem::{handler, IntoResponse};
use poem::web::{Data, Form, Html, Path};
use sqlx::{query_as, PgPool};

#[handler]
pub async fn location_types_view(Data(pool): Data<&PgPool>) -> impl IntoResponse {
    let location_types = query_as!(LocationType, "SELECT * FROM location_types")
        .fetch_all(pool)
        .await
        .unwrap();
    Html(LocationTypesTemplate { location_types }.render().unwrap())
}

#[handler]
pub async fn location_type_view(Path(id): Path<Id>, Data(pool): Data<&PgPool>) -> impl IntoResponse {
    let location_type = query_as!(
        LocationType,
        "SELECT * FROM location_types WHERE id = $1",
        id
    )
        .fetch_one(pool)
        .await
        .unwrap();
    Html(LocationTypeTemplate { location_type }.render().unwrap())
}

#[handler]
pub async fn location_type_update(
    Path(_id): Path<Id>,
    Form(location_type): Form<LocationType>,
    Data(pool): Data<&PgPool>,
) -> impl IntoResponse {
    query_as!(
        LocationType,
        "UPDATE location_types SET name = $1, description = $2 WHERE id = $3 RETURNING *",
        location_type.name,
        location_type.description,
        location_type.id
    )
        .fetch_one(pool)
        .await
        .unwrap();
    Html(LocationTypeTemplate { location_type }.render().unwrap())
}

#[handler]
pub async fn location_type_editor(Path(id): Path<Id>, Data(pool): Data<&PgPool>) -> impl IntoResponse {
    let location_type = query_as!(
        LocationType,
        "SELECT * FROM location_types WHERE id = $1",
        id
    )
        .fetch_one(pool)
        .await
        .unwrap();
    Html(
        LocationTypeEditorTemplate { location_type }
            .render()
            .unwrap(),
    )
}
