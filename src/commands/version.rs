use std::env;
use std::io::{self, Write};

use anyhow::{Context, Result};

pub fn run() -> Result<()> {
    println!("awesome-skills-cli v{}", env!("CARGO_PKG_VERSION"));
    io::stdout().flush().context("failed to flush stdout")?;
    Ok(())
}

pub fn current_platform_suffix() -> String {
    let os = match env::consts::OS {
        "macos" => "macos",
        other => other,
    };
    let arch = match env::consts::ARCH {
        "x86_64" => "x64",
        "aarch64" => "arm64",
        other => other,
    };
    format!("{os}-{arch}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn platform_suffix_matches_release_convention() {
        let suffix = current_platform_suffix();
        assert!(!suffix.is_empty());
        assert!(suffix.contains('-'));
    }
}
