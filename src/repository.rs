use std::collections::{BTreeSet, HashMap};
use std::sync::OnceLock;

use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use strsim::levenshtein;

const SKILLS_INDEX_JSON: &str = include_str!("../skills_index.json");
const ALIASES_JSON: &str = include_str!("../data/aliases.json");

include!(concat!(env!("OUT_DIR"), "/embedded_skills.rs"));

static REPOSITORY: OnceLock<Repository> = OnceLock::new();

#[derive(Debug, Deserialize)]
pub struct SkillEntry {
    pub id: String,
    #[allow(dead_code)]
    path: String,
    pub category: String,
    pub name: String,
    pub description: String,
    pub risk: String,
    pub source: String,
    pub date_added: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AliasFile {
    aliases: HashMap<String, String>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct CatalogEntry<'a> {
    pub id: &'a str,
    pub category: &'a str,
    pub description: &'a str,
    pub risk: &'a str,
}

#[derive(Debug)]
pub struct Repository {
    skills: Vec<SkillEntry>,
    aliases: HashMap<String, String>,
    contents: HashMap<String, &'static str>,
}

impl Repository {
    fn load() -> Result<Self> {
        let skills: Vec<SkillEntry> = serde_json::from_str(SKILLS_INDEX_JSON)
            .context("failed to parse embedded skills_index.json")?;
        let aliases = serde_json::from_str::<AliasFile>(ALIASES_JSON)
            .context("failed to parse embedded aliases.json")?
            .aliases;
        let contents = EMBEDDED_SKILLS
            .iter()
            .copied()
            .map(|(id, content)| (id.to_string(), content))
            .collect();

        Ok(Self {
            skills,
            aliases,
            contents,
        })
    }

    pub fn global() -> Result<&'static Self> {
        if let Some(repo) = REPOSITORY.get() {
            return Ok(repo);
        }
        let repo = Self::load()?;
        let _ = REPOSITORY.set(repo);
        Ok(REPOSITORY.get().unwrap())
    }

    pub fn all_skills(&self) -> &[SkillEntry] {
        &self.skills
    }

    pub fn categories(&self) -> Vec<&str> {
        let mut categories: BTreeSet<&str> = BTreeSet::new();
        for skill in &self.skills {
            categories.insert(&skill.category);
        }
        categories.into_iter().collect()
    }

    pub fn skills_by_category(&self, category: &str) -> Vec<&SkillEntry> {
        self.skills
            .iter()
            .filter(|skill| skill.category == category)
            .collect()
    }

    pub fn get_skill(&self, id: &str) -> Option<&SkillEntry> {
        let resolved = self.resolve_skill_id(id);
        self.skills.iter().find(|skill| skill.id == resolved)
    }

    pub fn get_skill_content(&self, id: &str) -> Option<&'static str> {
        let resolved = self.resolve_skill_id(id);
        self.contents.get(resolved).copied()
    }

    pub fn not_found_error(&self, id: &str) -> anyhow::Error {
        let suggestion = self.suggest_skill_id(id)
            .map(|s| format!("\nDid you mean: {s}?"))
            .unwrap_or_default();
        anyhow!("Skill not found: {id}{suggestion}")
    }

    pub fn search(&self, query: &str) -> Vec<&SkillEntry> {
        let query_lower = query.to_lowercase();
        let query_words: Vec<&str> = query_lower.split_whitespace().collect();
        let mut scored = self
            .skills
            .iter()
            .map(|skill| {
                let id = skill.id.to_lowercase();
                let name = skill.name.to_lowercase();
                let description = skill.description.to_lowercase();
                let category = skill.category.to_lowercase();
                let mut score = 0;

                if id == query_lower {
                    score += 100;
                } else if id.starts_with(&query_lower) {
                    score += 80;
                } else if id.contains(&query_lower) {
                    score += 60;
                }

                if name == query_lower {
                    score += 90;
                } else if name.starts_with(&query_lower) {
                    score += 70;
                } else if name.contains(&query_lower) {
                    score += 50;
                }

                if category == query_lower {
                    score += 40;
                } else if category.contains(&query_lower) {
                    score += 20;
                }

                if description.contains(&query_lower) {
                    score += 30;
                }

                for word in &query_words {
                    if id.contains(word) {
                        score += 10;
                    }
                    if description.contains(word) {
                        score += 5;
                    }
                }

                (skill, score)
            })
            .filter(|(_, score)| *score > 0)
            .collect::<Vec<_>>();

        scored.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.id.cmp(&b.0.id)));
        scored.into_iter().map(|(skill, _)| skill).collect()
    }

    pub fn suggest_skill_id(&self, id: &str) -> Option<&str> {
        if let Some((alias, canonical)) = self
            .aliases
            .iter()
            .find(|(alias, _)| alias.starts_with(id) || id.starts_with(alias.as_str()))
        {
            return Some(if alias.as_str() == id {
                canonical.as_str()
            } else {
                alias.as_str()
            });
        }

        self.skills
            .iter()
            .filter_map(|skill| {
                let distance = levenshtein(&id.to_lowercase(), &skill.id.to_lowercase());
                (distance <= 5).then_some((skill.id.as_str(), distance))
            })
            .min_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(b.0)))
            .map(|(skill_id, _)| skill_id)
    }

    fn resolve_skill_id<'a>(&'a self, id: &'a str) -> &'a str {
        self.aliases.get(id).map(String::as_str).unwrap_or(id)
    }
}

pub fn format_skill_row(id: &str, category: &str, description: &str, risk: &str) -> String {
    let desc = if description.chars().count() <= 60 {
        description.to_string()
    } else {
        let truncated: String = description.chars().take(57).collect();
        format!("{truncated}...")
    };
    format!("{id}\t{category}\t{desc}\t{risk}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repository_resolves_aliases() {
        let repo = Repository::global().expect("repository loads");
        let skill = repo
            .get_skill("finishing-a-branch")
            .expect("alias resolves");
        assert_eq!(skill.id, "finishing-a-development-branch");
    }

    #[test]
    fn format_skill_row_truncates_long_descriptions() {
        assert_eq!(
            format_skill_row("id", "cat", "short", "low"),
            "id\tcat\tshort\tlow"
        );
        assert_eq!(
            format_skill_row("id", "cat", "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz", "low"),
            "id\tcat\tabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcde...\tlow"
        );
    }
}
