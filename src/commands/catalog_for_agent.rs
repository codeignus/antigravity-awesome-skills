use anyhow::{Context, Result};
use serde_json::{json, Value};

use crate::output;
use crate::repository::Repository;

pub fn run(repository: &Repository, limit: usize, offset: usize) -> Result<()> {
    let skills = repository.all_skills();
    let total = skills.len();
    let entries: Vec<Value> = skills
        .iter()
        .skip(offset)
        .take(limit)
        .map(|skill| {
            json!({
                "id": skill.id,
                "category": skill.category,
                "description": skill.description,
                "risk": skill.risk,
            })
        })
        .collect();

    let returned = entries.len();
    let has_more = offset + returned < total;

    output::eprint(format_args!(
        "total={total} offset={offset} limit={limit} returned={returned} has_more={has_more}"
    ))?;

    let output =
        serde_json::to_string_pretty(&entries).context("failed to serialize catalog output")?;
    println!("{output}");
    Ok(())
}
