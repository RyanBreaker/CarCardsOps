use crate::models::{Id, Location, Waybill};
use crate::templates::waybills::{WaybillEditorTemplate, WaybillsTemplate};
use askama::Template;
use poem::web::{Data, Form, Html, Path, Redirect};
use poem::{handler, IntoResponse};
use sqlx::{query, query_as, SqlitePool};

#[handler]
pub async fn waybills_view(Data(pool): Data<&SqlitePool>) -> impl IntoResponse {
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
pub async fn waybill_new(Data(pool): Data<&SqlitePool>) -> impl IntoResponse {
    let locations = sqlx::query_as!(Location, "SELECT * FROM LOCATIONS").fetch_all(pool);
    let waybills = sqlx::query_as!(Waybill, "SELECT * FROM WAYBILLS").fetch_all(pool);
    let (locations, waybills) = tokio::try_join!(locations, waybills).unwrap();
    Html(
        WaybillEditorTemplate {
            waybill: Waybill::default(),
            waybills,
            locations,
            is_new: true,
        }
        .render()
        .unwrap(),
    )
}

#[handler]
pub async fn waybill_edit(Path(id): Path<Id>, Data(pool): Data<&SqlitePool>) -> impl IntoResponse {
    let waybill = query_as!(Waybill, "SELECT * FROM waybills WHERE id=$1", id).fetch_one(pool);
    let waybills = query_as!(Waybill, "SELECT * FROM waybills WHERE id <> $1", id).fetch_all(pool);
    let locations = query_as!(Location, "SELECT * FROM locations").fetch_all(pool);
    let (waybill, waybills, locations) = tokio::try_join!(waybill, waybills, locations).unwrap();
    Html(
        WaybillEditorTemplate {
            waybill,
            waybills,
            locations,
            is_new: false,
        }
        .render()
        .unwrap(),
    )
}

fn get_waybill_ids(waybill: &Waybill) -> (Option<Id>, Option<Id>) {
    (
        waybill.prev_waybill_id.filter(|id| *id > 0),
        waybill.next_waybill_id.filter(|id| *id > 0),
    )
}

#[handler]
pub async fn waybill_post(
    Form(waybill): Form<Waybill>,
    Data(pool): Data<&SqlitePool>,
) -> impl IntoResponse {
    let (prev_waybill_id, next_waybill_id) = get_waybill_ids(&waybill);
    query!(
        r#"
        INSERT INTO waybills (name, description, routing, from_location_id, to_location_id, prev_waybill_id, next_waybill_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        waybill.name,
        waybill.description,
        waybill.routing,
        waybill.from_location_id,
        waybill.to_location_id,
        prev_waybill_id,
        next_waybill_id,
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
    Data(pool): Data<&SqlitePool>,
) -> impl IntoResponse {
    let (prev_waybill_id, next_waybill_id) = get_waybill_ids(&waybill);
    query!(
        r#"
        UPDATE waybills
        SET name=$1, description=$2, routing=$3, from_location_id=$4, to_location_id=$5, prev_waybill_id=$6, next_waybill_id=$7
        WHERE id=$8
        "#,
        waybill.name,
        waybill.description,
        waybill.routing,
        waybill.from_location_id,
        waybill.to_location_id,
        prev_waybill_id,
        next_waybill_id,
        id
    )
        .execute(pool)
        .await
        .unwrap();
    Redirect::see_other("/waybills")
}
