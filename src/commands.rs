use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(name = "obsidian-nm", author = "solthodox", version)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Subcommands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Subcommands {
    Add(Add),
    NogAdd(Add), // TODO
}

/// Add an external note/template
#[derive(Debug, Clone, Parser)]
#[clap(long_about = "Add an external Obsidian note

You can easily add Obsidian notes or templates from other authors from remote git repositories.
- **Example of adding a note:**
  obsidiannm add git@github.com:josephmisiti/awesome-machine-learning.git

- **Example of adding a template:**
  obsidiannm add -t git@github.com:llZektorll/OB_Template.git")]
pub struct Add {
    // Git repository of the notes
    #[arg(value_name = "URL")]
    pub git_remote_url: Option<String>,

    // Git repository of the notes
    #[arg(value_name = "URL", requires = "git_remote_url")]
    pub note_path: Option<String>,

    // If its a template
    #[arg(short = 't', long, default_value_t = false)]
    pub template: bool,
}
