use serde::Deserialize;

pub mod get_stream;
pub mod playlist;
pub mod search;
pub mod track;
pub mod tracks;
pub mod user;

#[derive(Deserialize)]
pub struct SaveParams {
    pub save: bool,
}
