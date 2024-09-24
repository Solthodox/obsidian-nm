use std::path::PathBuf;

use commands::Subcommands;
use yansi::Paint as _;

pub mod commands;
pub mod errors;
pub mod note_downloader;
mod utils;
use dirs::document_dir;
use errors::ObsidianNmError;

#[tokio::main]
pub async fn run(command: Subcommands) -> Result<(), ObsidianNmError> {
    match command {
        Subcommands::Add(add) => {
            println!("{}", "🪨 Running ObsidianNM add 🪨".green());
            println!("{}", "Downloads a new note into your vault".green());
            let Some(git_remote_url) = add.git_remote_url else {
                todo!()
            };
            let is_template = add.template;
            if let Some(note_path) = add.note_path {
                let rename_as = add.rename;
                note_downloader::download_remote_note(
                    git_remote_url,
                    get_default_vault_path(),
                    note_path,
                    is_template,
                    if rename_as.is_some() {
                        rename_as.unwrap()
                    } else {
                        String::new()
                    },
                )
                .await
                .unwrap();
            }
        }
        _ => {}
    }
    Ok(())
}

fn get_default_vault_path() -> PathBuf {
    let vault_dir = document_dir()
        .map(|dir| dir.join("Obsidian Vault"))
        .unwrap();

    vault_dir
}
