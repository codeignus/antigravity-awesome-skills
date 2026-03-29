use std::collections::HashMap;
use std::sync::OnceLock;

include!(concat!(env!("OUT_DIR"), "/embedded_meta_skills.rs"));

static META_REPOSITORY: OnceLock<MetaRepository> = OnceLock::new();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MetaSkillEntry {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
}

const META_SKILLS: &[MetaSkillEntry] = &[
    MetaSkillEntry {
        id: "awesome-skills-cli",
        name: "awesome-skills-cli",
        description: "Offline command reference and setup guidance for the awesome-skills-cli binary.",
    },
    MetaSkillEntry {
        id: "recommend-awesome-skills",
        name: "recommend-awesome-skills",
        description: "Context-first recommendation workflow for choosing and optionally installing awesome skills.",
    },
];

#[derive(Debug)]
pub struct MetaRepository {
    contents: HashMap<&'static str, &'static str>,
}

impl MetaRepository {
    fn load() -> Self {
        let contents = EMBEDDED_META_SKILLS.iter().copied().collect();
        Self { contents }
    }

    pub fn global() -> &'static Self {
        META_REPOSITORY.get_or_init(Self::load)
    }

    pub fn all_skills(&self) -> &[MetaSkillEntry] {
        META_SKILLS
    }

    pub fn get_skill(&self, id: &str) -> Option<&MetaSkillEntry> {
        META_SKILLS.iter().find(|skill| skill.id == id)
    }

    pub fn get_skill_content(&self, id: &str) -> Option<&'static str> {
        self.contents.get(id).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meta_repository_embeds_expected_skills() {
        let repo = MetaRepository::global();
        let ids = repo
            .all_skills()
            .iter()
            .map(|skill| skill.id)
            .collect::<Vec<_>>();

        assert!(ids.contains(&"awesome-skills-cli"));
        assert!(ids.contains(&"recommend-awesome-skills"));
        assert_eq!(
            repo.get_skill("awesome-skills-cli").map(|skill| skill.name),
            Some("awesome-skills-cli")
        );
        assert!(repo.get_skill_content("awesome-skills-cli").is_some());
        assert!(repo.get_skill_content("recommend-awesome-skills").is_some());
    }
}
