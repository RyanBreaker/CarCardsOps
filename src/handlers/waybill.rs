use crate::models::{Id, Location, Waybill};
use crate::templates::waybills::{WaybillEditorTemplate, WaybillsTemplate};
use askama::Template;
use poem::web::{Data, Form, Html, Path, Redirect};
use poem::{handler, IntoResponse};
use sqlx::{query, query_as, PgPool};

#[handler]
pub async fn waybills_view(Data(pool): Data<&PgPool>) -> impl IntoResponse {
    let waybills = query_as!(
        Waybill,
        r#"
        SELECT *
        FROM waybills
        ORDER BY id
        "#
    )
    .fetch_all(pool)
    .await
    .unwrap();
    Html(WaybillsTemplate { waybills }.render().unwrap())
}

#[handler]
pub async fn waybill_new(Data(pool): Data<&PgPool>) -> impl IntoResponse {
    let locations = sqlx::query_as!(Location, "SELECT * FROM LOCATIONS")
        .fetch_all(pool)
        .await
        .unwrap();
    Html(
        WaybillEditorTemplate {
            waybill: Waybill::default(),
            locations,
        }
        .render()
        .unwrap(),
    )
}

#[handler]
pub async fn waybill_edit(Path(id): Path<Id>, Data(pool): Data<&PgPool>) -> impl IntoResponse {
    let waybill = query_as!(Waybill, "SELECT * FROM waybills WHERE id=$1", id).fetch_one(pool);
    let locations = query_as!(Location, "SELECT * FROM locations").fetch_all(pool);
    let (waybill, locations) = tokio::try_join!(waybill, locations).unwrap();
    Html(
        WaybillEditorTemplate { waybill, locations }
            .render()
            .unwrap(),
    )
}

#[handler]
pub async fn waybill_post(
    Form(waybill): Form<Waybill>,
    Data(pool): Data<&PgPool>,
) -> impl IntoResponse {
    query!(
        "INSERT INTO waybills (name, description, routing, from_location_id, to_location_id) VALUES ($1, $2, $3, $4, $5)",
        waybill.name,
        waybill.description,
        waybill.routing,
        waybill.from_location_id,
        waybill.to_location_id,
    )
        .execute(pool)
        .await
        .unwrap();
    Redirect::see_other("/waybills")
}

#[handler]
pub async fn waybill_put(
    Path(id): Path<Id>,
    Form(waybill): Form<Waybill>,
    Data(pool): Data<&PgPool>,
) -> impl IntoResponse {
    query!(
        "UPDATE waybills SET name=$1, description=$2, routing=$3, from_location_id=$4, to_location_id=$5 WHERE id=$6",
        waybill.name,
        waybill.description,
        waybill.routing,
        waybill.from_location_id,
        waybill.to_location_id,
        id
    )
        .execute(pool)
        .await
        .unwrap();
    Redirect::see_other("/waybills")
}
