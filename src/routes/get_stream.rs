use crate::AppState;
use crate::routes::SaveParams;
use axum::Json;
use axum::body::Body;
use axum::extract::{Path, Query, State};
use axum::http::Response;
use domain::errors::music_services::soundcloud_api_error::SoundcloudApiError;
use serde::Deserialize;
use tokio_util::io::ReaderStream;

#[derive(Deserialize)]
pub struct TrackData {
    media_url: Option<String>,
    track_token: Option<String>,
}

pub async fn stream(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(params): Query<SaveParams>,
    Json(payload): Json<TrackData>,
) -> Result<Response<Body>, SoundcloudApiError> {
    let s3_res = state
        .s3_client
        .get_object()
        .bucket(&state.s3_bucket_name)
        .key(&id)
        .send()
        .await;

    if let Ok(output) = s3_res {
        let reader = output.body.into_async_read();
        let stream = ReaderStream::new(reader);
        return Ok(Response::new(Body::from_stream(stream)));
    }

    let res = state
        .soundcloud
        .stream(id, params.save, payload.media_url, payload.track_token)
        .await?;

    Ok(Response::new(res))
}
