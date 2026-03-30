use std::cmp::Ordering;
use std::env;
use std::fs;
use std::io::{self, Read};
use std::mem;
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use serde::Deserialize;

use super::version::current_platform_suffix;
use crate::output;

pub fn run() -> Result<()> {
    let current_version = env!("CARGO_PKG_VERSION");
    let api_url = "https://api.github.com/repos/sickn33/antigravity-awesome-skills/releases/latest";
    let mut response = ureq::get(api_url)
        .header("User-Agent", "awesome-skills-cli")
        .call()
        .map_err(|e| anyhow::anyhow!("failed to check for updates: {e}"))?;

    let body_str = response
        .body_mut()
        .read_to_string()
        .map_err(|e| anyhow::anyhow!("failed to read response body: {e}"))?;
    let release: GitHubRelease =
        serde_json::from_str(&body_str).context("failed to parse GitHub release response")?;

    let suffix = current_platform_suffix();
    let plan = plan_update(&release, current_version, suffix)?;
    let UpdatePlan::Ready {
        latest_version,
        asset_name,
        download_url,
    } = plan
    else {
        output::eprint(format_args!(
            "You're already on the latest version (v{current_version})."
        ))?;
        return Ok(());
    };

    let current_exe = env::current_exe().context("failed to determine current executable")?;
    output::eprint(format_args!(
        "New version available: v{latest_version} (current: v{current_version})"
    ))?;
    output::eprint(format_args!(
        "Will download {asset_name} and replace {}",
        current_exe.display()
    ))?;
    output::print(format_args!("Continue? [y/N] "))?;
    output::flush().context("failed to flush prompt")?;

    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .context("failed to read input")?;
    let answer = answer.trim().to_lowercase();
    if answer != "y" && answer != "yes" {
        output::eprint(format_args!("Update cancelled."))?;
        return Ok(());
    }

    let mut download_response = ureq::get(&download_url)
        .header("User-Agent", "awesome-skills-cli")
        .call()
        .map_err(|e| anyhow::anyhow!("failed to download release asset: {e}"))?;
    let mut bytes = Vec::new();
    download_response
        .body_mut()
        .as_reader()
        .read_to_end(&mut bytes)
        .map_err(|e| anyhow::anyhow!("failed to read downloaded release asset: {e}"))?;

    let temp_path = current_exe.with_extension("download");
    let guard = TempFile(temp_path.clone());
    fs::write(&temp_path, bytes)
        .with_context(|| format!("failed to write {}", temp_path.display()))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = fs::metadata(&temp_path)
            .with_context(|| format!("failed to read metadata for {}", temp_path.display()))?
            .permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&temp_path, permissions)
            .with_context(|| format!("failed to set permissions on {}", temp_path.display()))?;
    }

    fs::rename(&temp_path, &current_exe)
        .with_context(|| format!("failed to replace {}", current_exe.display()))?;
    mem::forget(guard);
    output::eprint(format_args!("Updated to v{latest_version} successfully!"))?;
    Ok(())
}

struct TempFile(PathBuf);

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
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
    if compare_versions(latest_version, current_version) != Ordering::Greater {
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

fn compare_versions(a: &str, b: &str) -> Ordering {
    let left = a.split('.').map(|p| p.parse::<u32>().unwrap_or(0));
    let right = b.split('.').map(|p| p.parse::<u32>().unwrap_or(0));
    for (l, r) in left.zip(right).chain(std::iter::repeat((0, 0))).take(3) {
        match l.cmp(&r) {
            Ordering::Equal => continue,
            other => return other,
        }
    }
    Ordering::Equal
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
        assert_eq!(compare_versions("9.0.0", "9.0.0"), Ordering::Equal);
        assert_eq!(compare_versions("9.0.1", "9.0.0"), Ordering::Greater);
        assert_eq!(compare_versions("8.9.9", "9.0.0"), Ordering::Less);
    }
}
