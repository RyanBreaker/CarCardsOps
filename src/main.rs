mod api;
mod handlers;
mod models;
mod templates;

use crate::handlers::{location_types, locations};
use crate::models::{Location, LocationType};
use crate::templates::IndexTemplate;
use askama::Template;
use poem::listener::TcpListener;
use poem::middleware::Tracing;
use poem::web::{Data, Html};
use poem::{get, handler, EndpointExt, Route, Server};
use sqlx::PgPool;
use tracing::info;

#[handler]
async fn index(Data(pool): Data<&PgPool>) -> Html<String> {
    let (locations, location_types) =
        match tokio::try_join!(Location::all(pool), LocationType::all(pool)) {
            Ok(a) => a,
            Err(e) => panic!("{e}"),
        };
    let template = IndexTemplate {
        locations,
        location_types,
    };

    Html(template.render().unwrap())
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok();
    color_eyre::install().unwrap();
    tracing_subscriber::fmt::init();

    info!("Creating db pools...");
    let pool = db_init().await;

    info!("Initializing app...");
    let app = Route::new()
        .at("/", get(index))
        .nest("/locations", locations())
        .nest("/location_types", location_types())
        // .nest("/api", api()) // TODO: Will this be used?
        .data(pool)
        .with(Tracing);

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
