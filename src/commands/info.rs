use anyhow::{anyhow, bail, Result};

use crate::repository::Repository;

pub fn run(repository: &Repository, skill_id: &str) -> Result<()> {
    let skill = repository.get_skill(skill_id).ok_or_else(|| {
        let mut message = format!("Skill not found: {skill_id}");
        if let Some(suggestion) = repository.suggest_skill_id(skill_id) {
            message.push_str(&format!("\nDid you mean: {suggestion}?"));
        }
        anyhow!(message)
    })?;

    println!("ID:          {}", skill.id);
    println!("Name:        {}", skill.name);
    println!("Category:    {}", skill.category);
    println!("Description: {}", skill.description);
    println!("Risk:        {}", skill.risk);
    println!("Source:      {}", skill.source);
    if let Some(date_added) = &skill.date_added {
        println!("Date Added:  {date_added}");
    }

    println!("\n--- SKILL.md ---\n");
    match repository.get_skill_content(skill_id) {
        Some(content) => {
            print!("{content}");
            if !content.ends_with('\n') {
                println!();
            }
        }
        None => bail!("Skill content not available for: {skill_id}"),
    }

    Ok(())
}
