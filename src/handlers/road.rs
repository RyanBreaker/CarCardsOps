use crate::models::{Id, Road};
use crate::templates::roads::{RoadEditorTemplate, RoadsTemplate};
use askama::Template;
use poem::web::{Data, Form, Html, Path, Redirect};
use poem::{handler, IntoResponse};
use sqlx::{query, query_as, SqlitePool};

#[handler]
pub async fn roads_view(Data(pool): Data<&SqlitePool>) -> impl IntoResponse {
    let roads = query_as!(Road, "SELECT * FROM roads")
        .fetch_all(pool)
        .await
        .unwrap();
    Html(RoadsTemplate { roads }.render().unwrap())
}

#[handler]
pub async fn road_new() -> impl IntoResponse {
    Html(
        RoadEditorTemplate {
            road: Road::default(),
            is_new: true,
        }
        .render()
        .unwrap(),
    )
}

#[handler]
pub async fn road_editor(Path(id): Path<Id>, Data(pool): Data<&SqlitePool>) -> impl IntoResponse {
    let road = query_as!(Road, "SELECT * FROM roads WHERE id = $1", id,)
        .fetch_one(pool)
        .await
        .unwrap();
    Html(
        RoadEditorTemplate {
            road,
            is_new: false,
        }
        .render()
        .unwrap(),
    )
}

#[handler]
pub async fn road_post(Form(road): Form<Road>, Data(pool): Data<&SqlitePool>) -> impl IntoResponse {
    query!(
        "INSERT INTO roads (name, shorthand) VALUES ($1, $2)",
        road.name,
        road.shorthand
    )
    .execute(pool)
    .await
    .unwrap();
    Redirect::see_other("/roads")
}

#[handler]
pub async fn road_put(
    Path(id): Path<Id>,
    Form(road): Form<Road>,
    Data(pool): Data<&SqlitePool>,
) -> impl IntoResponse {
    query!(
        "UPDATE roads SET name=$1, shorthand=$2 WHERE id=$3",
        road.name,
        road.shorthand,
        id
    )
    .execute(pool)
    .await
    .unwrap();
    Redirect::see_other("/roads")
}
