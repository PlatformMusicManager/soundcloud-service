use crate::AppState;
use axum::{
    Json,
    extract::{Path, State},
};
use domain::errors::music_services::soundcloud_api_error::SoundcloudApiError;
use domain::models::music_api::artist::ApiArtist;
use domain::models::music_api::track::ApiTrack;
use soundcloud::models::track::Track;

pub async fn get_user_handler(
    State(data): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiArtist>, SoundcloudApiError> {
    let user = data.soundcloud.get_user(&id).await?;
    let api_artist: ApiArtist = user.into();
    Ok(Json(api_artist))
}

pub async fn get_user_tracks_handler(
    State(data): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Vec<ApiTrack>>, SoundcloudApiError> {
    let tracks = data.soundcloud.get_user_tracks(&id).await?;

    let api_tracks: Vec<ApiTrack> = tracks
        .into_iter()
        .filter_map(|track| match track {
            Track::Full(data) => Some(data.into()),
            Track::Stub(_) => None,
        })
        .collect();

    Ok(Json(api_tracks))
}

#[derive(serde::Serialize)]
pub struct ArtistDetails {
    pub artist: ApiArtist,
    pub tracks: Vec<ApiTrack>,
    pub playlists: Vec<domain::models::music_api::playlist::ApiPlaylist>,
}

pub async fn get_artist_details_handler(
    State(data): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ArtistDetails>, SoundcloudApiError> {
    let (user_res, tracks_res, playlists_res) = tokio::join!(
        data.soundcloud.get_user(&id),
        data.soundcloud.get_user_tracks(&id),
        data.soundcloud.get_user_playlists(&id)
    );

    let user = user_res?;
    let tracks = tracks_res?;
    let playlists = playlists_res?;

    let api_artist: ApiArtist = user.into();

    let api_tracks: Vec<ApiTrack> = tracks
        .into_iter()
        .filter_map(|track| match track {
            Track::Full(data) => Some(data.into()),
            Track::Stub(_) => None,
        })
        .collect();

    let api_playlists: Vec<domain::models::music_api::playlist::ApiPlaylist> =
        playlists.into_iter().map(|p| p.into()).collect();

    Ok(Json(ArtistDetails {
        artist: api_artist,
        tracks: api_tracks,
        playlists: api_playlists,
    }))
}
