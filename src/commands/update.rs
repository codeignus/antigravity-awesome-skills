use std::env;
use std::fs;
use std::io::{self, Write};

use anyhow::{anyhow, Context, Result};
use serde::Deserialize;

use super::version::current_platform_suffix;

pub fn run() -> Result<()> {
    let current_version = env!("CARGO_PKG_VERSION");
    let api_url = "https://api.github.com/repos/sickn33/antigravity-awesome-skills/releases/latest";
    let mut response = ureq::get(api_url)
        .header("User-Agent", "awesome-skills-cli")
        .call()
        .context("failed to check for updates")?;

    let release: GitHubRelease = response
        .body_mut()
        .read_json()
        .context("failed to parse GitHub release response")?;

    let suffix = current_platform_suffix();
    let plan = plan_update(&release, current_version, &suffix)?;
    let UpdatePlan::Ready {
        latest_version,
        asset_name,
        download_url,
    } = plan
    else {
        println!("You're already on the latest version (v{current_version}).");
        return Ok(());
    };

    let current_exe = env::current_exe().context("failed to determine current executable")?;
    println!("New version available: v{latest_version} (current: v{current_version})");
    println!(
        "Will download {asset_name} and replace {}",
        current_exe.display()
    );
    print!("Continue? [y/N] ");
    io::stdout().flush().context("failed to flush prompt")?;

    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .context("failed to read input")?;
    let answer = answer.trim().to_lowercase();
    if answer != "y" && answer != "yes" {
        println!("Update cancelled.");
        return Ok(());
    }

    let bytes = ureq::get(&download_url)
        .header("User-Agent", "awesome-skills-cli")
        .call()
        .context("failed to download release asset")?
        .body_mut()
        .read_to_vec()
        .context("failed to read downloaded release asset")?;

    let temp_path = current_exe.with_extension("download");
    fs::write(&temp_path, bytes)
        .with_context(|| format!("failed to write {}", temp_path.display()))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = fs::metadata(&temp_path)?.permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&temp_path, permissions)?;
    }

    fs::rename(&temp_path, &current_exe)
        .with_context(|| format!("failed to replace {}", current_exe.display()))?;
    println!("Updated to v{latest_version} successfully!");
    Ok(())
}

enum UpdatePlan {
    UpToDate,
    Ready {
        latest_version: String,
        asset_name: String,
        download_url: String,
    },
}

#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
    assets: Vec<GitHubAsset>,
}

#[derive(Debug, Deserialize)]
struct GitHubAsset {
    name: String,
    browser_download_url: String,
}

fn plan_update(release: &GitHubRelease, current_version: &str, suffix: &str) -> Result<UpdatePlan> {
    let latest_version = release.tag_name.trim_start_matches('v');
    if compare_versions(latest_version, current_version) <= 0 {
        return Ok(UpdatePlan::UpToDate);
    }

    let expected_name = format!("awesome-skills-cli-{suffix}");
    let asset = release
        .assets
        .iter()
        .find(|asset| asset.name == expected_name)
        .ok_or_else(|| anyhow!("No binary found for platform: {suffix}"))?;

    Ok(UpdatePlan::Ready {
        latest_version: latest_version.to_string(),
        asset_name: asset.name.clone(),
        download_url: asset.browser_download_url.clone(),
    })
}

fn compare_versions(a: &str, b: &str) -> i32 {
    let left = a.split('.').map(|part| part.parse::<u32>().unwrap_or(0));
    let right = b.split('.').map(|part| part.parse::<u32>().unwrap_or(0));

    for (left, right) in left.zip(right).chain(std::iter::repeat((0, 0))).take(3) {
        if left > right {
            return 1;
        }
        if left < right {
            return -1;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_plan_selects_expected_release_asset() {
        let release = GitHubRelease {
            tag_name: "v9.1.0".to_string(),
            assets: vec![GitHubAsset {
                name: "awesome-skills-cli-linux-x64".to_string(),
                browser_download_url: "https://example.com/linux".to_string(),
            }],
        };

        let plan = plan_update(&release, "9.0.0", "linux-x64").expect("plan computes");
        match plan {
            UpdatePlan::Ready {
                latest_version,
                asset_name,
                download_url,
            } => {
                assert_eq!(latest_version, "9.1.0");
                assert_eq!(asset_name, "awesome-skills-cli-linux-x64");
                assert_eq!(download_url, "https://example.com/linux");
            }
            UpdatePlan::UpToDate => panic!("expected update to be available"),
        }
    }

    #[test]
    fn compare_versions_orders_semver_triplets() {
        assert_eq!(compare_versions("9.0.0", "9.0.0"), 0);
        assert_eq!(compare_versions("9.0.1", "9.0.0"), 1);
        assert_eq!(compare_versions("8.9.9", "9.0.0"), -1);
    }
}
