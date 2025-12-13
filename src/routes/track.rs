use axum::extract::{Path, Query, State};
use axum::Json;
use domain::errors::app_error::AppError;
use domain::models::music_api::track::ApiTrack;
use soundcloud::models::track::TrackData;
use crate::AppState;
use crate::routes::SaveParams;

pub async fn track(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Query(params): Query<SaveParams>,
) -> Result<Json<ApiTrack>, AppError>
{
    let res: ApiTrack = match state.database.get_track_full_soundcloud(id).await? {
        Some(track) => {
            track.into()
        },
        None => {
            let track = state.soundcloud
                .get_track_data(&id.to_string())
                .await?;

            println!("{:?}", track);

            let track: TrackData  = track
                .try_into()?;

            let (track_i, author_i) = track.clone().into();


            if params.save {
                state.database.add_track_soundcloud(&track_i, &author_i).await?;
            }

            track.into()
        }
    };

    Ok(Json(res))
}