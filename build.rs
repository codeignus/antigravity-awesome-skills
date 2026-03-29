use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("manifest dir"));
    let skills_dir = manifest_dir.join("skills");
    let meta_skills_dir = manifest_dir.join("src/skills");
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    let embedded_skills_path = out_dir.join("embedded_skills.rs");
    let embedded_meta_skills_path = out_dir.join("embedded_meta_skills.rs");

    println!("cargo:rerun-if-changed={}", skills_dir.display());
    println!("cargo:rerun-if-changed={}", meta_skills_dir.display());
    println!(
        "cargo:rerun-if-changed={}",
        manifest_dir.join("skills_index.json").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        manifest_dir.join("data/aliases.json").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        manifest_dir.join("data/bundles.json").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        manifest_dir.join("data/editorial-bundles.json").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        manifest_dir.join("data/catalog.json").display()
    );

    write_embedded_skill_file(
        &manifest_dir,
        &skills_dir,
        &embedded_skills_path,
        "EMBEDDED_SKILLS",
    );
    write_embedded_skill_file(
        &manifest_dir,
        &meta_skills_dir,
        &embedded_meta_skills_path,
        "EMBEDDED_META_SKILLS",
    );
}

fn write_embedded_skill_file(
    manifest_dir: &Path,
    skills_dir: &Path,
    generated_path: &Path,
    static_name: &str,
) {
    let entries = collect_skill_entries(manifest_dir, skills_dir);
    let mut generated = format!("pub static {static_name}: &[(&str, &str)] = &[\n");

    for (id, relative_path) in entries {
        generated.push_str(&format!(
            "    ({id:?}, include_str!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/{relative_path}\"))),\n"
        ));
    }

    generated.push_str("];\n");
    fs::write(generated_path, generated)
        .unwrap_or_else(|err| panic!("failed to write {}: {err}", generated_path.display()));
}

fn collect_skill_entries(manifest_dir: &Path, skills_dir: &Path) -> Vec<(String, String)> {
    let mut entries = Vec::new();
    let mut stack = vec![skills_dir.to_path_buf()];

    while let Some(dir) = stack.pop() {
        let read_dir = fs::read_dir(&dir).unwrap_or_else(|err| {
            panic!("failed to read {}: {err}", dir.display());
        });

        for item in read_dir {
            let item = item.expect("dir entry");
            let path = item.path();

            if path.is_dir() {
                stack.push(path);
                continue;
            }

            if path.file_name().and_then(|name| name.to_str()) == Some("SKILL.md") {
                let skill_dir = path.parent().expect("skill dir");
                let relative = skill_dir
                    .strip_prefix(skills_dir)
                    .expect("relative skill path")
                    .to_string_lossy()
                    .replace('\\', "/");
                let relative_path = path
                    .strip_prefix(manifest_dir)
                    .expect("skill file path")
                    .to_string_lossy()
                    .replace('\\', "/");
                entries.push((relative, relative_path));
            }
        }
    }

    entries.sort_by(|a, b| a.0.cmp(&b.0));
    entries
}
