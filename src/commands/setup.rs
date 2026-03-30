use std::path::Path;

use anyhow::{bail, Context, Result};

use crate::meta_repository::MetaRepository;
use crate::skill_io;

pub fn run(skill_ids: &[String], path: &Path) -> Result<()> {
    let repository = MetaRepository::global();
    let skills = if skill_ids.is_empty() {
        repository.all_skills().iter().collect::<Vec<_>>()
    } else {
        let mut errors = Vec::new();
        let mut resolved = Vec::new();

        for skill_id in skill_ids {
            let Some(skill) = repository.get_skill(skill_id) else {
                errors.push(format!("Meta skill not found: {skill_id}"));
                continue;
            };
            resolved.push(skill);
        }

        if !errors.is_empty() {
            bail!(errors.join("\n"));
        }

        resolved
    };

    for skill in skills {
        let content = repository
            .get_skill_content(skill.id)
            .with_context(|| format!("meta skill content not available for: {}", skill.id))?;

        skill_io::write_skill(path, skill.id, skill.name, content)?;
    }

    Ok(())
}
