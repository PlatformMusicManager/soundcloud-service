use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::Response;
use axum::response::IntoResponse;
use crate::AppState;

pub async fn get_stream_by_id(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    let res = state.soundcloud.stream_and_save(id).await.unwrap();

    Response::new(Body::new(res))
}