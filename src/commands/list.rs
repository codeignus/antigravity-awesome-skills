use anyhow::Result;

use crate::repository::{truncate, Repository};

pub fn run(repository: &Repository, category: Option<&str>) -> Result<()> {
    let skills = if let Some(category) = category {
        repository.skills_by_category(category)
    } else {
        repository.all_skills().iter().collect()
    };

    if skills.is_empty() {
        if let Some(category) = category {
            println!("No skills found in category \"{category}\".");
            println!(
                "Available categories: {}",
                repository.categories().join(", ")
            );
        } else {
            println!("No skills available.");
        }
        return Ok(());
    }

    for skill in &skills {
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
        "{} skill{} total.",
        skills.len(),
        if skills.len() == 1 { "" } else { "s" }
    );
    if category.is_none() {
        println!("Categories: {}", repository.categories().join(", "));
    }
    Ok(())
}
