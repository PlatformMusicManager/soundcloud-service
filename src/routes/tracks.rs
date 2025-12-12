use axum::extract::{Path, Query, State};
use axum::Json;
use domain::errors::music_services::soundcloud_api_error::SoundcloudApiError;
use domain::models::music_api::track::ApiTrack;
use serde::Deserialize;
use crate::AppState;

#[derive(Deserialize)]
struct TracksParams {
    ids: Vec<i64>,
}

// pub async fn tracks(
//     State(state): State<AppState>,
//     Query(params): Query<TracksParams>
// ) -> Result<Json<ApiTrack>, SoundcloudApiError>
// {
//     let found_in_db =
//         state.database.get_tracks_full_soundcloud(&params.ids).await?;
//
//     // let res = state.soundcloud.get_tracks_data(&ids).await?;
//
//     Ok(Json(res.try_into()?))
// }