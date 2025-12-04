use axum::extract::{Query, State};
use axum::Json;
use domain::errors::music_services::soundcloud_api_error::SoundcloudApiError;
use serde::Deserialize;
use soundcloud::models::search::SearchResponse;

use crate::AppState;

#[derive(Deserialize)]
pub struct SearchParams {
    query: String,
    offset: String,
    limit: String,
}

pub async fn search(State(state): State<AppState>, Query(params): Query<SearchParams>)
-> Result<Json<SearchResponse>, SoundcloudApiError>
{
    let res = state.soundcloud.search(&params.query, &params.offset, &params.limit).await?;

    Ok(Json(res))
}