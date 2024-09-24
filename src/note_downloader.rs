use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    errors,
    utils::{format_path, get_file_name_from_path},
};
use errors::DownloadError;
use git2::Repository;
use tempfile::TempDir;

pub type Result<T> = std::result::Result<T, DownloadError>;

pub async fn download_remote_note(
    url: String,
    vault_path: PathBuf,
    note_path: String,
    as_template: bool,
    rename_as: String,
) -> Result<()> {
    // Only md files are supported
    let note_path = format_path(&note_path, as_template);

    let vault_path = Path::new(&vault_path);

    // Create a temporary directory
    let temp_dir = TempDir::new().unwrap();

    // Get the path as a string
    let temp_path = temp_dir.path().to_str().unwrap();

    // Clone the repository into the temporary directory
    let _ = Repository::clone(&url, temp_path).unwrap();

    // Construct the source path for the file to copy
    let source_file_path = temp_dir.path().join(&note_path);

    // Determine the destination directory: either "Template" or "Notes"
    let folder = if as_template { "Template" } else { "Notes" };

    // Construct the destination directory path and create it if it doesn't exist
    let destination_dir = vault_path.join(folder);
    if !destination_dir.exists() {
        fs::create_dir_all(&destination_dir).unwrap();
    }

    let new_note_name = if rename_as.is_empty() {
        get_file_name_from_path(&note_path).unwrap()
    } else {
        format_path(&rename_as, false)
    };

    // Construct the full destination file path
    let destination_file_path =
        destination_dir.join(Path::new(&new_note_name).file_name().unwrap());

    // Copy the file from the cloned repo to the appropriate subdirectory in the vault
    fs::copy(&source_file_path, &destination_file_path).unwrap();

    println!(
        "Successfully copied the file to {}",
        destination_file_path.display()
    );

    // The TempDir will be automatically cleaned up when it goes out of scope
    Ok(())
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_download_remote_note_with_md_extension() {
        // This is a public repository for testing purposes
        let test_url = "https://github.com/thuquant/awesome-quant.git".to_string();

        // Attempt to download the remote note
        let result = download_remote_note(
            test_url,
            PathBuf::from("vault".to_string()),
            "README.md".to_string(),
            false,
            String::new(),
        )
        .await;

        // Check that the result is Ok
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_download_remote_note_no_md_extension() {
        // This is a public repository for testing purposes
        let test_url = "https://github.com/thuquant/awesome-quant.git".to_string();

        // Attempt to download the remote note
        let result = download_remote_note(
            test_url,
            PathBuf::from("vault".to_string()),
            "README".to_string(),
            false,
            String::new(),
        )
        .await;

        // Check that the result is Ok
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_download_template() {
        // This is a public repository for testing purposes
        let test_url = "https://github.com/llZektorll/OB_Template.git".to_string();

        // Attempt to download the remote note
        let result = download_remote_note(
            test_url,
            PathBuf::from("vault".to_string()),
            "0A_Templates/0A_16_Investment/0A_16_1_Stocks".to_string(),
            true,
            String::new(),
        )
        .await;

        // Check that the result is Ok
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_download_template_invalid_extension() {
        // This is a public repository for testing purposes
        let test_url = "https://github.com/thuquant/awesome-quant.git".to_string();

        // Attempt to download the remote note
        let result = download_remote_note(
            test_url,
            PathBuf::from("vault".to_string()),
            "README".to_string(),
            false,
            String::new(),
        )
        .await;

        // Check that the result is Ok
        assert!(result.is_ok());
    }
}
