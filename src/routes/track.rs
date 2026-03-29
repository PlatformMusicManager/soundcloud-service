use crate::AppState;
use crate::routes::SaveParams;
use axum::Json;
use axum::extract::{Path, Query, State};
use domain::errors::app_error::AppError;
use domain::models::music_api::track::ApiTrack;
use soundcloud::models::track::TrackData;

pub async fn track(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Query(params): Query<SaveParams>,
) -> Result<Json<ApiTrack>, AppError> {
    let res: ApiTrack = match state.database.get_track_full_soundcloud(id).await? {
        Some(track) => track.into(),
        None => {
            let track = state.soundcloud.get_track_data(&id.to_string()).await?;

            println!("{:?}", track);

            let track: TrackData = track.try_into()?;

            if params.save {
                let (track_i, author_i) = track.clone().into();
                state
                    .database
                    .add_track_soundcloud(&track_i, &author_i)
                    .await?;
            }

            track.into()
        }
    };

    Ok(Json(res))
}
