use crate::models::{Id, Location, Waybill};
use crate::templates::waybills::{WaybillEditorTemplate, WaybillLocation, WaybillsTemplate};
use askama::Template;
use futures::future::join_all;
use poem::web::{Data, Form, Html, Path, Redirect};
use poem::{handler, IntoResponse};
use sqlx::{query, query_as, SqlitePool};

async fn get_waybills_locations(waybill: &Waybill, pool: &SqlitePool) -> WaybillLocation {
    let from_location = query_as!(
        Location,
        "SELECT * FROM locations WHERE id=$1",
        waybill.from_location_id
    )
    .fetch_one(pool)
    .await
    .unwrap();
    let to_location = query_as!(
        Location,
        "SELECT * FROM locations WHERE id=$1",
        waybill.to_location_id
    )
    .fetch_one(pool)
    .await
    .unwrap();
    WaybillLocation {
        waybill: waybill.clone(),
        from_location,
        to_location,
    }
}

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
    let waybills: Vec<_> = waybills
        .iter()
        .map(|waybill| get_waybills_locations(waybill, pool))
        .collect();
    let waybills = join_all(waybills).await;

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
pub async fn waybill_editor(
    Path(id): Path<Id>,
    Data(pool): Data<&SqlitePool>,
) -> impl IntoResponse {
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

fn next_waybill_id(waybill: &Waybill) -> Option<Id> {
    waybill.next_waybill_id.filter(|id| *id > 0)
}

#[handler]
pub async fn waybill_post(
    Form(waybill): Form<Waybill>,
    Data(pool): Data<&SqlitePool>,
) -> impl IntoResponse {
    let next_waybill_id = next_waybill_id(&waybill);
    query!(
        r#"
        INSERT INTO waybills (consignee, description, routing, from_location_id, to_location_id, next_waybill_id)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        waybill.consignee,
        waybill.description,
        waybill.routing,
        waybill.from_location_id,
        waybill.to_location_id,
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
    let next_waybill_id = next_waybill_id(&waybill);
    query!(
        r#"
        UPDATE waybills
        SET consignee=$1, description=$2, routing=$3, via=$4, shipper=$5, from_location_id=$6, to_location_id=$7, next_waybill_id=$8
        WHERE id=$9
        "#,
        waybill.consignee,
        waybill.description,
        waybill.routing,
        waybill.via,
        waybill.shipper,
        waybill.from_location_id,
        waybill.to_location_id,
        next_waybill_id,
        id
    )
        .execute(pool)
        .await
        .unwrap();
    Redirect::see_other("/waybills")
}
