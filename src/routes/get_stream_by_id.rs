use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::Response;
use axum::response::IntoResponse;
use crate::AppState;
use tokio_util::io::ReaderStream;

pub async fn get_stream_by_id(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    let s3_res = state.s3_client
        .get_object()
        .bucket(&state.s3_bucket_name)
        .key(&id)
        .send()
        .await;

    if let Ok(output) = s3_res {
        let reader = output.body.into_async_read();
        let stream = ReaderStream::new(reader);
        return Response::new(Body::from_stream(stream));
    }

    let res = state.soundcloud.stream_and_save(id).await.unwrap();

    Response::new(Body::new(res))
}