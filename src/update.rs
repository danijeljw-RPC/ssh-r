use anyhow::{Context, Result};
use semver::Version;
use serde::Deserialize;

#[derive(Deserialize)]
struct Release { tag_name: String }

pub fn check_updates() -> Result<Option<(String, String)>> {
    // Prefer SSHR_REPO, fall back to SSHC_REPO, then default
    let repo = std::env::var("SSHR_REPO")
        .or_else(|_| std::env::var("SSHC_REPO"))
        .unwrap_or_else(|_| "repasscloud/ssh-r".to_string());
    let url = format!("https://api.github.com/repos/{repo}/releases/latest");

    let client = reqwest::blocking::Client::builder()
        .user_agent(format!("ssh-r/{}", env!("CARGO_PKG_VERSION")))
        .build()?;

    let rel: Release = client.get(url)
        .send()?.error_for_status()?
        .json()
        .context("invalid release payload")?;

    let latest = rel.tag_name.trim_start_matches('v').to_string();
    let current = env!("CARGO_PKG_VERSION").to_string();

    if Version::parse(&latest).ok() > Version::parse(&current).ok() {
        Ok(Some((current, latest)))
    } else {
        Ok(None)
    }
}
