use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

pub fn write_skill(path: &Path, id: &str, _name: &str, content: &str) -> Result<()> {
    let skill_dir = path.join(id);
    fs::create_dir_all(&skill_dir)
        .with_context(|| format!("failed to create {}", skill_dir.display()))?;
    fs::write(skill_dir.join("SKILL.md"), content)
        .with_context(|| format!("failed to write {}", skill_dir.join("SKILL.md").display()))?;
    Ok(())
}
