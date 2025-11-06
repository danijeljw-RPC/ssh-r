use anyhow::{bail, Context, Result};
use directories::BaseDirs;
use std::{env, fs, path::PathBuf};
use crate::model::{Config, HostEntry};

const DOT_PREFERRED: &str = ".ssh-r";  // primary for ssh-r
const DOT_COMPAT1: &str = ".ssh-c";    // compat with old name
const DOT_COMPAT2: &str = ".shh-c";    // compat with original typo

pub fn config_dir() -> Result<PathBuf> {
    // Prefer SSHR_DIR, fall back to SSHC_DIR for compatibility
    if let Ok(p) = env::var("SSHR_DIR").or_else(|_| env::var("SSHC_DIR")) {
        return Ok(PathBuf::from(p));
    }
    let base = BaseDirs::new().context("cannot resolve home directory")?;
    let home = base.home_dir();
    let preferred = home.join(DOT_PREFERRED);
    let compat1 = home.join(DOT_COMPAT1);
    let compat2 = home.join(DOT_COMPAT2);

    if preferred.exists() { Ok(preferred) }
    else if compat1.exists() { Ok(compat1) }
    else if compat2.exists() { Ok(compat2) }
    else { Ok(preferred) } // create preferred path
}

pub fn config_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("config.json"))
}

pub fn load_or_init() -> Result<Config> {
    let dir = config_dir()?;
    if !dir.exists() {
        fs::create_dir_all(&dir).with_context(|| format!("create {:?}", dir))?;
    }
    let path = dir.join("config.json");
    if !path.exists() {
        let cfg = Config::default();
        save(&cfg)?;
        return Ok(cfg);
    }
    let bytes = fs::read(&path).with_context(|| format!("read {:?}", path))?;
    let cfg: Config = serde_json::from_slice(&bytes).with_context(|| "parse config.json")?;
    Ok(cfg)
}

pub fn save(cfg: &Config) -> Result<()> {
    let path = config_path()?;
    let pretty = serde_json::to_vec_pretty(cfg)?;
    fs::write(&path, pretty).with_context(|| format!("write {:?}", path))?;
    Ok(())
}

pub fn upsert_host(mut cfg: Config, entry: HostEntry) -> Result<Config> {
    if let Some(idx) = cfg.hosts.iter().position(|h| h.name == entry.name) {
        cfg.hosts[idx] = entry;
    } else {
        cfg.hosts.push(entry);
    }
    Ok(cfg)
}

pub fn remove_host(mut cfg: Config, name: &str) -> Result<Config> {
    let before = cfg.hosts.len();
    cfg.hosts.retain(|h| h.name != name);
    if cfg.hosts.len() == before {
        bail!("alias '{}' not found", name);
    }
    Ok(cfg)
}

pub fn find_host(cfg: &Config, name: &str) -> Result<HostEntry> {
    cfg.hosts.iter().find(|h| h.name == name)
        .cloned()
        .with_context(|| format!("alias '{}' not found", name))
}
