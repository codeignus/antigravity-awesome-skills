use anyhow::Result;

use crate::output;

pub fn run() -> Result<()> {
    let version = env!("CARGO_PKG_VERSION");
    output::eprint(format_args!("awesome-skills-cli v{version}"))?;
    Ok(())
}

pub fn current_platform_suffix() -> &'static str {
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    { "linux-x64" }
    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    { "linux-arm64" }
    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    { "macos-x64" }
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    { "macos-arm64" }
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    { "windows-x64.exe" }
    #[cfg(not(any(
        all(target_os = "linux", target_arch = "x86_64"),
        all(target_os = "linux", target_arch = "aarch64"),
        all(target_os = "macos", target_arch = "x86_64"),
        all(target_os = "macos", target_arch = "aarch64"),
        all(target_os = "windows", target_arch = "x86_64"),
    )))]
    { "unknown" }
}
