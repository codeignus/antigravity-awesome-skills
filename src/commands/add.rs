use std::fs;
use std::path::Path;

use anyhow::{bail, Context, Result};

use crate::repository::Repository;

pub fn run(repository: &Repository, skill_ids: &[String], path: &Path) -> Result<()> {
    let mut errors = Vec::new();

    for skill_id in skill_ids {
        let Some(skill) = repository.get_skill(skill_id) else {
            let mut message = format!("Skill not found: {skill_id}");
            if let Some(suggestion) = repository.suggest_skill_id(skill_id) {
                message.push_str(&format!("\nDid you mean: {suggestion}?"));
            }
            errors.push(message);
            continue;
        };

        let Some(content) = repository.get_skill_content(skill_id) else {
            errors.push(format!("Skill content not available for: {}", skill.id));
            continue;
        };

        let skill_dir = path.join(&skill.id);
        fs::create_dir_all(&skill_dir)
            .with_context(|| format!("failed to create {}", skill_dir.display()))?;
        fs::write(skill_dir.join("SKILL.md"), content)
            .with_context(|| format!("failed to write {}", skill_dir.join("SKILL.md").display()))?;
        println!(
            "Added \"{}\" ({}) to {}/SKILL.md",
            skill.name,
            skill.id,
            skill_dir.display()
        );
    }

    if errors.is_empty() {
        Ok(())
    } else {
        bail!(errors.join("\n"))
    }
}
