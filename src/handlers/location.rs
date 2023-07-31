use crate::models::{Id, Location, LocationType};
use crate::templates::locations::{LocationEditorTemplate, LocationTemplate, LocationsTemplate};
use askama::Template;
use poem::http::StatusCode;
use poem::web::{Data, Form, Html, Path};
use poem::{handler, IntoResponse};
use sqlx::{query, query_as, PgPool};

#[handler]
pub async fn location_view(Path(id): Path<Id>, Data(pool): Data<&PgPool>) -> impl IntoResponse {
    let location = query_as!(Location, "SELECT * FROM locations WHERE id = $1", id)
        .fetch_one(pool)
        .await
        .unwrap();
    Html(LocationTemplate { location }.render().unwrap())
}

#[handler]
pub async fn locations_view(Data(pool): Data<&PgPool>) -> impl IntoResponse {
    let locations = query_as!(Location, "SELECT * FROM locations").fetch_all(pool);
    let location_types = query_as!(LocationType, "SELECT * FROM location_types").fetch_all(pool);
    let (Ok(locations), Ok(location_types)) = tokio::join!(locations, location_types) else {
        panic!("Failed to load locations or location types.");
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
    query!(
        "INSERT INTO locations (name, description, location_type_id) VALUES ($1, $2, $3)",
        location.name,
        location.description,
        location.location_type_id
    )
        .execute(pool)
        .await
        .unwrap();
    StatusCode::OK.with_header("HX-Refresh", "true")
}

#[handler]
pub async fn location_update(
    Path(id): Path<Id>,
    Form(location): Form<Location>,
    Data(pool): Data<&PgPool>,
) -> impl IntoResponse {
    let location = query_as!(
        Location,
        "UPDATE locations SET name = $1, description = $2 WHERE id = $3 RETURNING *",
        location.name,
        location.description,
        id,
    )
        .fetch_one(pool)
        .await
        .unwrap();
    Html(LocationTemplate { location }.render().unwrap())
}

#[handler]
pub async fn location_edit(Path(id): Path<Id>, Data(pool): Data<&PgPool>) -> impl IntoResponse {
    let location = query_as!(Location, "SELECT * FROM locations WHERE id = $1", id).fetch_one(pool);
    let location_types = query_as!(LocationType, "SELECT * FROM location_types").fetch_all(pool);
    let (location, location_types) = tokio::join!(location, location_types);
    let location = location.unwrap();
    let location_types = location_types.unwrap();
    Html(LocationEditorTemplate { location, location_types}.render().unwrap())
}

#[handler]
pub async fn location_creator() -> impl IntoResponse {
    Html("".to_string())
}
