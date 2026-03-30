use anyhow::Result;

use crate::output;
use crate::repository::Repository;

pub fn run(repository: &Repository, skill_id: &str) -> Result<()> {
    let skill = repository
        .get_skill(skill_id)
        .ok_or_else(|| repository.not_found_error(skill_id))?;

    output::eprint(format_args!("ID:          {}", skill.id))?;
    output::eprint(format_args!("Name:        {}", skill.name))?;
    output::eprint(format_args!("Category:    {}", skill.category))?;
    output::eprint(format_args!("Description: {}", skill.description))?;
    output::eprint(format_args!("Risk:        {}", skill.risk))?;
    output::eprint(format_args!("Source:      {}", skill.source))?;
    if let Some(date_added) = &skill.date_added {
        output::eprint(format_args!("Date added:  {date_added}"))?;
    }

    output::eprint(format_args!(""))?;

    let content = repository
        .get_skill_content(skill_id)
        .ok_or_else(|| anyhow::anyhow!("Skill content not available for: {skill_id}"))?;

    output::print(format_args!("{content}"))?;
    if !content.ends_with('\n') {
        output::print(format_args!(""))?;
    }
    Ok(())
}
