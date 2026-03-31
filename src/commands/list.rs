use anyhow::Result;

use crate::output;
use crate::repository::Repository;

pub fn run(
    repository: &Repository,
    category: Option<&str>,
    limit: Option<usize>,
    offset: usize,
) -> Result<()> {
    let skills = if let Some(category) = category {
        repository.skills_by_category(category)
    } else {
        repository.all_skills().iter().collect()
    };

    let total = skills.len();
    let page: Vec<_> = skills
        .into_iter()
        .skip(offset)
        .take(limit.unwrap_or(usize::MAX))
        .collect();
    let returned = page.len();
    let has_more = offset + returned < total;
    let effective_limit = limit.unwrap_or(returned);

    output::print(format_args!("id | category | risk | description"))?;
    for skill in &page {
        let desc = skill.description.replace('\n', " ");
        output::print(format_args!(
            "{} | {} | {} | {}",
            skill.id, skill.category, skill.risk, desc
        ))?;
    }

    output::eprint(format_args!(
        "total={total} offset={offset} limit={effective_limit} returned={returned} has_more={has_more}"
    ))?;

    Ok(())
}
