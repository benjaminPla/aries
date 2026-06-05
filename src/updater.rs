const CURRENT: &str  = env!("CARGO_PKG_VERSION");
const OWNER:   &str  = "benjaminPla";
const REPO:    &str  = "babushka";

/// Returns Some(version_string) if a newer release exists on GitHub.
/// Silently returns None on any network or parse error.
pub fn check() -> Option<String> {
    let agent = ureq::AgentBuilder::new()
        .timeout_read(std::time::Duration::from_secs(5))
        .timeout_connect(std::time::Duration::from_secs(3))
        .build();

    let body: serde_json::Value = agent
        .get(&format!("https://api.github.com/repos/{OWNER}/{REPO}/releases/latest"))
        .set("User-Agent", "babushka-app")
        .call()
        .ok()?
        .into_json()
        .ok()?;

    let tag     = body["tag_name"].as_str()?;
    let remote  = tag.trim_start_matches('v');

    match self_update::version::bump_is_greater(CURRENT, remote) {
        Ok(true) => {
            log::info!("[updater] nueva versión disponible: {remote} (actual: {CURRENT})");
            Some(remote.to_owned())
        }
        _ => None,
    }
}

/// Downloads and installs the latest release, replacing the running binary.
/// The caller should prompt the user to restart after this returns Ok.
pub fn apply() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    self_update::backends::github::Update::configure()
        .repo_owner(OWNER)
        .repo_name(REPO)
        .bin_name("babushka")
        .target("windows-x86_64")
        .bin_path_in_archive("babushka-windows-x86_64.exe")
        .current_version(CURRENT)
        .no_confirm(true)
        .show_download_progress(false)
        .build()?
        .update()?;
    log::info!("[updater] actualización aplicada correctamente");
    Ok(())
}
