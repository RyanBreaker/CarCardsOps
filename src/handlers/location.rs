use crate::models::{Id, Location, LocationType};
use crate::templates::locations::{LocationEditorTemplate, LocationsTemplate};
use askama::Template;
use poem::web::{Data, Form, Html, Path, Redirect};
use poem::{handler, IntoResponse};
use sqlx::{query, query_as, SqlitePool};

#[handler]
pub async fn locations_view(Data(pool): Data<&SqlitePool>) -> impl IntoResponse {
    let locations = query_as!(Location, "SELECT * FROM locations").fetch_all(pool);
    let location_types = query_as!(LocationType, "SELECT * FROM location_types").fetch_all(pool);
    let (locations, location_types) = tokio::try_join!(locations, location_types).unwrap();
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
pub async fn location_post(
    Form(location): Form<Location>,
    Data(pool): Data<&SqlitePool>,
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
    Redirect::see_other("/locations")
}

#[handler]
pub async fn location_put(
    Path(id): Path<Id>,
    Form(location): Form<Location>,
    Data(pool): Data<&SqlitePool>,
) -> impl IntoResponse {
    query!(
        "UPDATE locations SET name = $1, description = $2, location_type_id=$3 WHERE id = $4",
        location.name,
        location.description,
        location.location_type_id,
        id,
    )
    .execute(pool)
    .await
    .unwrap();
    Redirect::see_other("/locations")
}

#[handler]
pub async fn location_edit(Path(id): Path<Id>, Data(pool): Data<&SqlitePool>) -> impl IntoResponse {
    let location = query_as!(Location, "SELECT * FROM locations WHERE id = $1", id).fetch_one(pool);
    let location_types = query_as!(LocationType, "SELECT * FROM location_types").fetch_all(pool);
    let (location, location_types) = tokio::try_join!(location, location_types).unwrap();
    Html(
        LocationEditorTemplate {
            location,
            location_types,
        }
        .render()
        .unwrap(),
    )
}

#[handler]
pub async fn location_new(Data(pool): Data<&SqlitePool>) -> impl IntoResponse {
    let location = Location::default();
    let location_types = query_as!(LocationType, "SELECT * FROM location_types")
        .fetch_all(pool)
        .await
        .unwrap();
    Html(
        LocationEditorTemplate {
            location,
            location_types,
        }
        .render()
        .unwrap(),
    )
}
