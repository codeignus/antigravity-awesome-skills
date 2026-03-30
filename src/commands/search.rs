use anyhow::Result;

use crate::output;
use crate::repository::{format_skill_row, Repository};

pub fn run(repository: &Repository, query: &str) -> Result<()> {
    let results = repository.search(query);
    if results.is_empty() {
        output::eprint(format_args!("No skills matching \"{query}\"."))?;
        return Ok(());
    }

    for skill in &results {
        output::eprint(format_args!(
            "{}",
            format_skill_row(
                &skill.id,
                &skill.category,
                &skill.description,
                &skill.risk
            )
        ))?;
    }

    output::eblank_line()?;
    output::eprint(format_args!(
        "{} result{} for \"{query}\".",
        results.len(),
        if results.len() == 1 { "" } else { "s" }
    ))?;
    Ok(())
}
