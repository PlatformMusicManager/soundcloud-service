mod routes;

use crate::routes::get_stream::stream;
use crate::routes::playlist::playlist;
use crate::routes::search::search;
use crate::routes::track::track;
use aws_sdk_s3::Client;
use axum::Router;
use axum::routing::{get, post};
use cache_lib::RedisClient;
use chrono::Duration;
use database_lib::client::PostgresDb;
use dotenv::dotenv;
use soundcloud::soundcloud_client::SoundCloudApi;
use std::env;
use std::sync::Arc;
use utils_lib::auth_layer::{AuthLayer, AuthState};
use utils_lib::create_s3_client::new_s3_client;
use utils_lib::jwt::JwtClient;
use utils_lib::parse_env::parse_env;

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

    let redis_url = parse_env("REDIS_URL");
    let user_cache_ttl: i64 = parse_env("USER_CACHE_TTL");
    let session_cache_ttl: u64 = parse_env("SESSION_CACHE_TTL");
    let verify_ttl_s: u64 = parse_env("VERIFY_TTL");
    let verify_ttl = Duration::seconds(verify_ttl_s as i64);
    let verify_attempts: u8 = parse_env("VERIFY_ATTEMPTS");

    let jwt_secret = parse_env("JWT_SECRET");

    let access_token_ttl_s: i64 = parse_env("ACCESS_TOKEN_TTL");
    let refresh_token_ttl_s: i64 = parse_env("REFRESH_TOKEN_TTL");

    let database_url: String = env::var("DATABASE_URL").unwrap();

    let database = PostgresDb::new(database_url, Duration::new(0, 0).unwrap()).await;

    let auth_state = AuthState {
        database: database.clone(),
        redis: RedisClient::new(
            redis_url,
            session_cache_ttl,
            user_cache_ttl,
            verify_ttl_s,
            verify_attempts,
        ),
        jwt: Arc::new(JwtClient::new(
            jwt_secret,
            verify_ttl,
            Duration::seconds(access_token_ttl_s),
            Duration::seconds(refresh_token_ttl_s),
        )),
    };

    let soundcloud_id: String = env::var("SOUNDCLOUD_ID").unwrap();

    let s3_url: String = env::var("S3_URL").unwrap();

    let s3_login: String = env::var("S3_LOGIN").unwrap();
    let s3_pass: String = env::var("S3_PASS").unwrap();
    let s3_bucket_name: String = env::var("S3_BUCKET_NAME").unwrap();

    let s3_client = new_s3_client(s3_url, s3_login, s3_pass, vec![&s3_bucket_name]).await;

    let app_state = AppState {
        soundcloud: Arc::new(
            SoundCloudApi::new(
                soundcloud_id,
                s3_client.clone(),
                s3_bucket_name.clone(),
                6 * 1024 * 1024, // Default part size: 6MB
            )
            .await,
        ),
        database,
        s3_client,
        s3_bucket_name,
    };

    let routes = Router::new()
        .route("/search", get(search))
        .route("/track/{id}", get(track))
        .route("/stream/{id}", post(stream))
        .route("/playlist/{id}", get(playlist))
        .with_state(app_state);

    let app = Router::new()
        .nest("/api/soundcloud", routes)
        .layer(AuthLayer { state: auth_state });

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
