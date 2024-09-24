use thiserror::Error;

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("error during git operation: {0}")]
    GitError(String),
}
