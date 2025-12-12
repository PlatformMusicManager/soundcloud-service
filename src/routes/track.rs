use axum::extract::{Path, State};
use axum::Json;
use domain::errors::app_error::AppError;
use domain::errors::music_services::soundcloud_api_error::SoundcloudApiError;
use domain::errors::music_services::soundcloud_api_error::SoundcloudApiError::TrackDataIsNotFull;
use domain::models::music_api::track::ApiTrack;
use soundcloud::models::track::Track;
use crate::AppState;

pub async fn track(
    State(state): State<AppState>,
    Path(id): Path<i64>
) -> Result<Json<ApiTrack>, AppError>
{
    let res: ApiTrack = match state.database.get_track_full_soundcloud(id).await? {
        Some(track) => {
            track.into()
        },
        None => {
            state.soundcloud
                .get_track_data(&id.to_string())
                .await?
                .try_into()?
        }
    };

    Ok(Json(res))
}