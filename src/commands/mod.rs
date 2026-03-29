pub mod add;
pub mod catalog_for_agent;
pub mod info;
pub mod list;
pub mod search;
pub mod setup;
pub mod update;
pub mod version;

use std::path::PathBuf;

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Commands {
    List {
        #[arg(long)]
        category: Option<String>,
    },
    Search {
        query: String,
    },
    CatalogForAgent,
    Info {
        skill_id: String,
    },
    Add {
        #[arg(required = true)]
        skill_ids: Vec<String>,
        #[arg(long)]
        path: PathBuf,
    },
    Setup {
        skill_ids: Vec<String>,
        #[arg(long)]
        path: PathBuf,
    },
    Update,
    Version,
}
