use axum::body::Body;
use axum::extract::{Path, Query, State};
use axum::http::Response;
use axum::Json;
use serde::Deserialize;
use soundcloud::soundcloud_client::SoundcloudError;
use tokio_util::io::ReaderStream;
use crate::AppState;
use crate::routes::StreamParams;

#[derive(Deserialize)]
pub struct TrackData {
    media_url: String,
    track_token: String,
}

pub async fn get_stream_by_token(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(params): Query<StreamParams>,
    Json(payload): Json<TrackData>
) -> Result<Response<Body>, SoundcloudError> {
    let s3_res = state.s3_client
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

    if params.save {
        let res = state.soundcloud.stream_and_save_by_token(id, payload.media_url, payload.track_token).await?;
        Ok(Response::new(Body::new(res)))
    } else {
        let res = state.soundcloud.stream_by_token(payload.media_url, payload.track_token).await?;
        Ok(Response::new(Body::from_stream(res)))
    }
}