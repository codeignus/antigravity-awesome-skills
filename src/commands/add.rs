use std::path::Path;

use anyhow::{bail, Result};

use crate::repository::Repository;
use crate::skill_io;

pub fn run(repository: &Repository, skill_ids: &[String], path: &Path) -> Result<()> {
    let mut errors = Vec::new();

    for skill_id in skill_ids {
        let Some(skill) = repository.get_skill(skill_id) else {
            errors.push(repository.not_found_error(skill_id).to_string());
            continue;
        };

        let Some(content) = repository.get_skill_content(skill_id) else {
            errors.push(format!("Skill content not available for: {}", skill.id));
            continue;
        };

        if let Err(e) = skill_io::write_skill(path, &skill.id, &skill.name, content) {
            errors.push(e.to_string());
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        bail!(errors.join("\n"))
    }
}
