mod commands;
mod meta_repository;
mod output;
mod repository;
mod skill_io;

use std::io;

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
    if let Err(err) = run() {
        if is_broken_pipe(&err) {
            std::process::exit(0);
        }
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn is_broken_pipe(err: &anyhow::Error) -> bool {
    err.chain().any(|cause| {
        cause
            .downcast_ref::<io::Error>()
            .is_some_and(|io_err| io_err.kind() == io::ErrorKind::BrokenPipe)
    })
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    let repo = Repository::global()?;

    match cli.command {
        Commands::List {
            category,
            limit,
            offset,
        } => commands::list::run(repo, category.as_deref(), limit, offset),
        Commands::Search { query } => commands::search::run(repo, &query),
        Commands::Info { skill_id } => commands::info::run(repo, &skill_id),
        Commands::Add { skill_ids, path } => commands::add::run(repo, &skill_ids, &path),
        Commands::Setup { skill_ids, path } => commands::setup::run(&skill_ids, &path),
        Commands::Update => commands::update::run(),
        Commands::Version => commands::version::run(),
    }
}
