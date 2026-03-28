mod commands;
mod repository;

use std::env;
use std::process;

use anyhow::Result;
use clap::Parser;

use commands::Commands;
use repository::Repository;

#[derive(Parser, Debug)]
#[command(name = "awesome-skills-cli")]
#[command(about = "Offline-first awesome-skills CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let raw_args: Vec<String> = env::args().collect();
    if let Some(subcommand) = raw_args.get(1)
        && subcommand == "install"
    {
        eprintln!("The install command was renamed. Use `awesome-skills-cli add --path <dir> <skill-id...>` instead.");
        process::exit(1);
    }

    if let Err(err) = run() {
        eprintln!("{err}");
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    let repo = Repository::global()?;

    match cli.command {
        Commands::List { category } => commands::list::run(&repo, category.as_deref()),
        Commands::Search { query } => commands::search::run(&repo, &query),
        Commands::CatalogForAgent => commands::catalog_for_agent::run(&repo),
        Commands::Info { skill_id } => commands::info::run(&repo, &skill_id),
        Commands::Add { skill_ids, path } => commands::add::run(&repo, &skill_ids, &path),
        Commands::Update => commands::update::run(),
        Commands::Version => commands::version::run(),
    }
}
