use serde::Deserialize;

pub mod get_stream_by_id;
pub mod get_stream_by_token;
pub mod search;

#[derive(Deserialize)]
pub struct StreamParams {
    save: bool,
}