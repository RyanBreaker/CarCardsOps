mod models;
mod templates;

use crate::models::{Location, LocationType};
use crate::templates::HomeTemplate;
use askama::Template;
use poem::http::StatusCode;
use poem::listener::TcpListener;
use poem::web::{Data, Form, Html};
use poem::{get, handler, post, EndpointExt, Route, Server};
use serde::Deserialize;
use sqlx::PgPool;
use tracing::info;

#[handler]
async fn index(Data(pool): Data<&PgPool>) -> Html<String> {
    let location_types = LocationType::all(pool).await.unwrap();
    let locations = Location::all(pool).await.unwrap();
    let template = HomeTemplate {
        locations,
        location_types,
    };

    Html(template.render().unwrap())
}

#[derive(Deserialize)]
struct Params {
    name: String,
    description: String,
    location_type_id: i32,
}

#[handler]
async fn new_location(query: Form<Params>, Data(pool): Data<&PgPool>) -> StatusCode {
    let location = Location::new(
        query.name.clone(),
        query.description.clone(),
        query.location_type_id,
    );
    location.insert(pool).await.unwrap();

    StatusCode::CREATED
}

fn api() -> Route {
    Route::new().at("/locations", post(new_location))
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok();
    color_eyre::install().unwrap();
    tracing_subscriber::fmt::init();

    info!("Creating db pools...");
    let pool = db_init().await;

    let app = Route::new()
        .at("/", get(index))
        .at("/locations", post(new_location))
        .nest("/api", api())
        .data(pool);

    Server::new(TcpListener::bind("localhost:3000"))
        .run(app)
        .await?;

    Ok(())
}

async fn db_init() -> PgPool {
    let db_url = &std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::connect(db_url)
        .await
        .expect("Failed to connect to Postgres")
}
