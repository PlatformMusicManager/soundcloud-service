mod routes;
pub mod s3_client;

use std::env;
use std::fmt::Debug;
use std::str::FromStr;
use std::sync::Arc;
use aws_sdk_s3::Client;
use axum::Router;
use axum::routing::{get, post};
use chrono::Duration;
use database_lib::client::PostgresDb;
use dotenv::dotenv;
use soundcloud::soundcloud_client::SoundCloudApi;
use crate::routes::get_stream::stream;
use crate::routes::playlist::playlist;
use crate::routes::search::search;
use crate::s3_client::new_s3_client;

pub fn parse_env<T>(key: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    match env::var(key) {
        Ok(value) => {
            match value.parse::<T>() {
                Ok(parsed_value) => parsed_value,
                Err(e) => panic!("Failed to parse environment variable '{}': {:?}", key, e),
            }
        }
        Err(e) => panic!("Environment variable '{}' not set: {}", key, e),
    }
}

#[derive(Clone)]
struct AppState {
    pub soundcloud: Arc<SoundCloudApi>,
    pub database: PostgresDb,
    pub s3_client: Client,
    pub s3_bucket_name: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let soundcloud_id: String = env::var("SOUNDCLOUD_ID").unwrap();

    let s3_url: String = env::var("S3_URL").unwrap();

    let s3_login: String = env::var("S3_LOGIN").unwrap();
    let s3_pass: String = env::var("S3_PASS").unwrap();
    let s3_bucket_name: String = env::var("S3_BUCKET_NAME").unwrap();

    let database_url: String = env::var("DATABASE_URL").unwrap();


    let s3_client = new_s3_client(s3_url, s3_login, s3_pass, vec![&s3_bucket_name]).await;

    let app_state = AppState {
        soundcloud: Arc::new(SoundCloudApi::new(
            soundcloud_id,
            s3_client.clone(),
            s3_bucket_name.clone(),
            6 * 1024 * 1024 // Default part size: 6MB
        ).await),
        database: PostgresDb::new(database_url, Duration::new(0, 0).unwrap()).await,
        s3_client,
        s3_bucket_name
    };


    let app = Router::new()
        .route("/search", get(search))
        .route("/stream/{id}", post(stream))
        .route("/playlist/{id}", get(playlist))

        .with_state(app_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}