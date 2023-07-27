use crate::models::{Id, Location};
use poem::http::StatusCode;
use poem::web::{Data, Form, Path};
use poem::{handler, post, put, Route};
use sqlx::PgPool;

#[allow(unused)]
pub fn api() -> Route {
    Route::new()
        .at("/locations", post(create_location))
        .at("/locations/:id", put(edit_location))
}

#[handler]
async fn create_location(
    Form(new_location): Form<Location>,
    Data(pool): Data<&PgPool>,
) -> StatusCode {
    new_location.insert(pool).await.unwrap();
    StatusCode::CREATED
}

#[handler]
async fn edit_location(
    Path(_id): Path<Id>,
    Form(location): Form<Location>,
    Data(pool): Data<&PgPool>,
) -> StatusCode {
    location.update(pool).await.unwrap();
    StatusCode::OK
}
