use anyhow::Result;

use crate::repository::{truncate, Repository};

pub fn run(repository: &Repository, query: &str) -> Result<()> {
    let results = repository.search(query);
    if results.is_empty() {
        println!("No skills matching \"{query}\".");
        return Ok(());
    }

    for skill in &results {
        println!(
            "{}\t{}\t{}\t{}",
            skill.id,
            skill.category,
            truncate(&skill.description, 60),
            skill.risk
        );
    }

    println!();
    println!(
        "{} result{} for \"{query}\".",
        results.len(),
        if results.len() == 1 { "" } else { "s" }
    );
    Ok(())
}
