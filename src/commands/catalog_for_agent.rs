use anyhow::Result;

use crate::repository::{CatalogEntry, Repository};

pub fn run(repository: &Repository) -> Result<()> {
    let condensed = repository
        .all_skills()
        .iter()
        .map(|skill| CatalogEntry {
            id: &skill.id,
            category: &skill.category,
            description: &skill.description,
            risk: &skill.risk,
        })
        .collect::<Vec<_>>();

    println!("{}", serde_json::to_string(&condensed)?);
    Ok(())
}
