use serde::Deserialize;

pub mod get_stream;
pub mod search;
pub mod playlist;
pub mod track;
pub mod tracks;

#[derive(Deserialize)]
pub struct SaveParams {
    save: bool,
}