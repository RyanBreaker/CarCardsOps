mod handlers;
mod models;
mod templates;

use crate::handlers::{location_types, locations, waybills};
use crate::models::{Location, LocationType};
use crate::templates::IndexTemplate;
use askama::Template;
use poem::listener::TcpListener;
use poem::middleware::Tracing;
use poem::web::{Data, Html};
use poem::{get, handler, EndpointExt, IntoResponse, Route, Server};
use sqlx::{query_as, SqlitePool};
use tokio::try_join;
use tracing::info;

#[handler]
async fn index(Data(pool): Data<&SqlitePool>) -> impl IntoResponse {
    let locations = query_as!(Location, "SELECT * FROM locations").fetch_all(pool);
    let location_types = query_as!(LocationType, "SELECT * FROM location_types").fetch_all(pool);
    let (locations, location_types) = match try_join!(locations, location_types) {
        Ok(a) => a,
        Err(e) => panic!("{e}"),
    };

    Html(
        IndexTemplate {
            locations,
            location_types,
        }
        .render()
        .unwrap(),
    )
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok();
    color_eyre::install().unwrap();
    tracing_subscriber::fmt::init();

    info!("Creating db pools...");
    let pool = db_init().await;
    sqlx::migrate!().run(&pool).await.unwrap();

    info!("Initializing app...");
    let app = Route::new()
        .at("/", get(index))
        .nest("/location_types", location_types())
        .nest("/locations", locations())
        .nest("/waybills", waybills())
        .data(pool)
        .with(Tracing);

    Server::new(TcpListener::bind("localhost:3000"))
        .run(app)
        .await?;

    Ok(())
}

async fn db_init() -> SqlitePool {
    let db_url = &std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqlitePool::connect(db_url)
        .await
        .expect("Failed to connect to Postgres")
}
