use thiserror::Error;

#[derive(Error, Debug)]
pub enum ObsidianNmError {
    #[error("error during downloading ({dep}): {source}")]
    DownloadError { dep: String, source: DownloadError },
}

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("error during git operation: {0}")]
    GitError(String),
}
