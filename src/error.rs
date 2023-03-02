use std::fmt::Debug;
use std::io;
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to parse info.dat file.")]
    InfoParsingFailed(#[from] serde_json::Error),
    #[error("Failed to parse a beatmap file. (new format: {err_as_new}, old format: {err_as_old})")]
    BeatmapParsingFailed {
        err_as_new: serde_json::Error,
        err_as_old: serde_json::Error
    },
    #[error(transparent)]
    IOError(#[from] io::Error)
}


