use askama::Template;
use poem::handler;
use poem::web::{Data, Form, Html, Path};
use sqlx::PgPool;
use crate::models::{Id, LocationType};
use crate::templates::location_types::{LocationTypeEditorTemplate, LocationTypesTemplate, LocationTypeTemplate};

#[handler]
pub async fn location_types_view(Data(pool): Data<&PgPool>) -> Html<String> {
    let location_types = LocationTypesTemplate {
        location_types: LocationType::all(pool).await.unwrap(),
    };
    Html(location_types.render().unwrap())
}

#[handler]
pub async fn location_type_view(Path(id): Path<i32>, Data(pool): Data<&PgPool>) -> Html<String> {
    let location_type = LocationTypeTemplate {
        location_type: LocationType::select(id, pool).await.unwrap(),
    };
    Html(location_type.render().unwrap())
}

#[handler]
pub async fn location_type_update(
    Path(_id): Path<Id>,
    Form(location_type): Form<LocationType>,
    Data(pool): Data<&PgPool>,
) -> Html<String> {
    location_type.update(pool).await.unwrap();
    Html(LocationTypeTemplate { location_type }.render().unwrap())
}

#[handler]
pub async fn location_type_editor(Path(id): Path<Id>, Data(pool): Data<&PgPool>) -> Html<String> {
    let location_type = LocationType::select(id, pool).await.unwrap();
    Html(
        LocationTypeEditorTemplate { location_type }
            .render()
            .unwrap(),
    )
}
