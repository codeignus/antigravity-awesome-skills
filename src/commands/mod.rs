pub mod add;
pub mod info;
pub mod list;
pub mod search;
pub mod setup;
pub mod update;
pub mod version;

use std::path::PathBuf;

use anyhow::{anyhow, Result};
use clap::Subcommand;

fn parse_limit(value: &str) -> Result<usize> {
    let limit = value
        .parse::<usize>()
        .map_err(|_| anyhow!("invalid value '{value}' for '--limit'"))?;

    if limit == 0 {
        return Err(anyhow!("--limit must be at least 1"));
    }

    Ok(limit)
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    List {
        #[arg(long)]
        category: Option<String>,
        #[arg(long, value_parser = parse_limit)]
        limit: Option<usize>,
        #[arg(long, default_value_t = 0)]
        offset: usize,
    },
    Search {
        query: String,
    },
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
