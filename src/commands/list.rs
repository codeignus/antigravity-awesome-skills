use anyhow::Result;

use crate::output;
use crate::repository::{format_skill_row, Repository};

pub fn run(repository: &Repository, category: Option<&str>) -> Result<()> {
    let skills = if let Some(category) = category {
        repository.skills_by_category(category)
    } else {
        repository.all_skills().iter().collect()
    };

    if skills.is_empty() {
        if let Some(category) = category {
            output::eprint(format_args!("No skills found in category \"{category}\"."))?;
            output::eprint(format_args!(
                "Available categories: {}",
                repository.categories().join(", ")
            ))?;
        } else {
            output::eprint(format_args!("No skills available."))?;
        }
        return Ok(());
    }

    for skill in &skills {
        output::eprint(format_args!(
            "{}",
            format_skill_row(&skill.id, &skill.category, &skill.description, &skill.risk)
        ))?;
    }

    output::eblank_line()?;
    output::eprint(format_args!(
        "{} skill{} total.",
        skills.len(),
        if skills.len() == 1 { "" } else { "s" }
    ))?;
    if category.is_none() {
        let categories = repository.categories();
        output::eprint(format_args!(
            "Categories: {}",
            categories.join(", ")
        ))?;
    }
    Ok(())
}
