use std::collections::{BTreeSet, HashMap};
use std::sync::OnceLock;

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use strsim::levenshtein;

const SKILLS_INDEX_JSON: &str = include_str!("../skills_index.json");
const ALIASES_JSON: &str = include_str!("../data/aliases.json");
const BUNDLES_JSON: &str = include_str!("../data/bundles.json");
const EDITORIAL_BUNDLES_JSON: &str = include_str!("../data/editorial-bundles.json");
const CATALOG_JSON: &str = include_str!("../data/catalog.json");

include!(concat!(env!("OUT_DIR"), "/embedded_skills.rs"));

static REPOSITORY: OnceLock<Repository> = OnceLock::new();

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SkillEntry {
    pub id: String,
    pub path: String,
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

#[derive(Debug, Serialize)]
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
        let _: serde_json::Value =
            serde_json::from_str(BUNDLES_JSON).context("failed to parse embedded bundles.json")?;
        let _: serde_json::Value = serde_json::from_str(EDITORIAL_BUNDLES_JSON)
            .context("failed to parse embedded editorial-bundles.json")?;
        let _: serde_json::Value =
            serde_json::from_str(CATALOG_JSON).context("failed to parse embedded catalog.json")?;
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
        REPOSITORY
            .get()
            .ok_or_else(|| anyhow!("failed to initialize repository"))
    }

    pub fn all_skills(&self) -> &[SkillEntry] {
        &self.skills
    }

    pub fn categories(&self) -> Vec<String> {
        let mut categories = BTreeSet::new();
        for skill in &self.skills {
            categories.insert(skill.category.clone());
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
            return Some(if alias == &id {
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

pub fn truncate(value: &str, max_len: usize) -> String {
    if value.chars().count() <= max_len {
        return value.to_string();
    }

    let truncated = value
        .chars()
        .take(max_len.saturating_sub(3))
        .collect::<String>();
    format!("{truncated}...")
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
    fn repository_embeds_supporting_json_files() {
        let bundles: serde_json::Value = serde_json::from_str(BUNDLES_JSON).expect("bundles parse");
        let editorial: serde_json::Value =
            serde_json::from_str(EDITORIAL_BUNDLES_JSON).expect("editorial bundles parse");
        let catalog: serde_json::Value =
            serde_json::from_str(CATALOG_JSON).expect("catalog parses");

        assert!(!bundles.is_null());
        assert!(!editorial.is_null());
        assert!(!catalog.is_null());
    }

    #[test]
    fn truncate_adds_ellipsis_when_needed() {
        assert_eq!(truncate("short", 10), "short");
        assert_eq!(truncate("abcdefghijklmnopqrstuvwxyz", 10), "abcdefg...");
    }
}
