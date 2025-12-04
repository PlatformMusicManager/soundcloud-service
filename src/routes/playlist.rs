use axum::extract::{Path, State};
use axum::Json;
use domain::errors::app_error::AppError;
use domain::models::db::soundcloud::{CreateReplacePlaylistInput, FullPlaylistResponse};
use domain::models::music_api::playlist::ApiPlaylist;
use crate::AppState;

pub async fn playlist(
    State(state): State<AppState>,
    Path(id): Path<i64>
) -> Result<Json<ApiPlaylist>, AppError> {
    match state.database.get_playlist_soundcloud(id).await? {
        Some(pl) => {
            Ok(Json(pl.into()))
        }
        None => {
            let playlist = state.soundcloud
                .get_playlist(&id.to_string())
                .await?;

            let playlist_create: CreateReplacePlaylistInput = playlist.clone().into();

            match state.database.replace_or_create_playlist_soundcloud(
                &playlist_create.playlist,
                &playlist_create.playlist_author,
                &playlist_create.tracks,
                &playlist_create.track_authors
            ).await {
                Ok(_) => {},
                Err(e) => {eprintln!("ERROR: Can't add playlist to db, bks of{}", e);}
            };

            Ok(Json(playlist.into()))
        }
    }
}