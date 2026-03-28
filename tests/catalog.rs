use assert_cmd::Command;
use serde_json::Value;

fn cargo_bin() -> Command {
    Command::cargo_bin("awesome-skills-cli").expect("binary builds for integration tests")
}

#[test]
fn catalog_for_agent_emits_condensed_json_array() {
    let output = cargo_bin()
        .arg("catalog-for-agent")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let parsed: Value =
        serde_json::from_slice(&output).expect("catalog-for-agent should emit JSON");
    let entries = parsed
        .as_array()
        .expect("catalog-for-agent should emit a JSON array");
    let skill = entries
        .iter()
        .find(|entry| entry.get("id") == Some(&Value::String("ab-test-setup".to_string())))
        .expect("catalog includes a known skill");

    let object = skill.as_object().expect("catalog entries are JSON objects");

    assert_eq!(object.len(), 4, "catalog entries stay condensed");
    assert_eq!(
        object.get("category"),
        Some(&Value::String("marketing".to_string()))
    );
    assert_eq!(
        object.get("risk"),
        Some(&Value::String("unknown".to_string()))
    );
    assert!(object
        .get("description")
        .and_then(Value::as_str)
        .is_some_and(|description| description.contains("A/B test")));
}

#[test]
fn catalog_for_agent_count_matches_skills_index() {
    let catalog_output = cargo_bin()
        .arg("catalog-for-agent")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let catalog: Vec<Value> = serde_json::from_slice(&catalog_output).expect("catalog JSON");

    let index: Value = serde_json::from_str(include_str!("../skills_index.json"))
        .expect("skills_index.json parses");
    let index_count = index.as_array().expect("skills_index is array").len();

    assert_eq!(
        catalog.len(),
        index_count,
        "catalog-for-agent entry count should match skills_index.json skill count"
    );
}
