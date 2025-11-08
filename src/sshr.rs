use anyhow::{Context, Result};
use std::{process::Command, path::PathBuf};
use which::which;
use crate::model::{HostEntry, Auth};

fn expand_tilde(p: &str) -> String {
    if let Some(rest) = p.strip_prefix("~/") {
        if let Some(home) = directories::BaseDirs::new().map(|b| b.home_dir().to_path_buf()) {
            return home.join(rest).to_string_lossy().into_owned();
        }
    }
    p.to_string()
}

fn ssh_bin() -> Result<PathBuf> {
    which("ssh").or_else(|_| which("ssh.exe")).context("ssh not found in PATH")
}

fn sshpass_bin() -> Option<PathBuf> {
    which("sshpass").ok()
}

pub fn export_command(h: &HostEntry, extra: &[String]) -> String {
    let mut parts: Vec<String> = vec!["ssh".into()];
    if h.port != 22 {
        parts.push("-p".into()); parts.push(h.port.to_string());
    }
    match &h.auth {
        Auth::Cert{ identity_file } => {
            parts.push("-i".into());
            parts.push(expand_tilde(identity_file));
        }
        Auth::Password{ .. } => {}
    }
    parts.extend(extra.iter().cloned());
    parts.push(format!("{}@{}", h.user, h.host));
    parts.join(" ")
}

pub fn connect(h: &HostEntry, extra: &[String]) -> Result<i32> {
    let ssh = ssh_bin()?;
    let target = format!("{}@{}", h.user, h.host);

    let mut args: Vec<String> = vec![];
    if h.port != 22 {
        args.push("-p".into()); args.push(h.port.to_string());
    }
    if let Auth::Cert{ identity_file } = &h.auth {
        args.push("-i".into());
        args.push(expand_tilde(identity_file));
    }
    args.extend(extra.iter().cloned());
    args.push(target);

    if let Auth::Password{ password } = &h.auth {
        if let Some(pass) = password.as_ref() {
            if let Some(sshpass) = sshpass_bin() {
                let status = Command::new(sshpass)
                    .arg("-p").arg(pass)
                    .arg(ssh.as_os_str())
                    .args(&args)
                    .status()
                    .with_context(|| "failed to spawn sshpass/ssh")?;
                return Ok(status.code().unwrap_or(-1));
            }
        }
    }

    let status = Command::new(ssh)
        .args(&args)
        .status()
        .with_context(|| "failed to spawn ssh")?;
    Ok(status.code().unwrap_or(-1))
}
