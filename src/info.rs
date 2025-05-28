use anyhow::{Context, Result};
use std::{env::var, fs::read_to_string, process::Command};

fn cmd(parts: &[&str]) -> Option<String> {
    let out = Command::new(parts.first().unwrap())
        .args(parts.iter().skip(1))
        .output()
        .ok()?
        .stdout;
    let out = out.trim_ascii();

    if out.is_empty() {
        None
    } else {
        Some(String::from_utf8_lossy(out).to_string())
    }
}

fn user() -> Option<String> {
    var("USER")
        .ok()
        .map(|s| s.trim_ascii().to_owned())
        .map_or(cmd(&["whoami"]), Some)
}

fn host() -> Option<String> {
    if let Ok(h) = read_to_string("/etc/hostname") {
        Some(h.trim_ascii().to_owned())
    } else if let Ok(h) = var("HOSTNAME") {
        Some(h.trim_ascii().to_owned())
    } else {
        cmd(&["hostname"]).map_or_else(|| cmd(&["uname", "-r"]), Some)
    }
}

fn title() -> String {
    let user = user().unwrap_or("unknown".to_owned());
    let host = host().unwrap_or("unknown".to_owned());
    format!("{user}@{host}")
}

fn kernel() -> Result<String> {
    Ok(read_to_string("/proc/version")
        .context("Couldn't read /proc/version")?
        .split_whitespace()
        .nth(2)
        .context("Failed to get kernel version")?
        .to_owned())
}

fn shell() -> Option<String> {
    let shell = if let Ok(shell) = var("SHELL") {
        Some(shell)
    } else {
        cmd(&["readlink", "/bin/sh"])
    };

    shell.map(|sh| {
        sh.rsplit_once('/')
            .map(|(_, sh)| sh.to_owned())
            .unwrap_or(sh)
    })
}

fn distro() -> Result<String> {
    read_to_string("/etc/os-release")
        .context("Couldn't read /etc/os-release")?
        .lines()
        .find(|l| l.starts_with("PRETTY_NAME="))
        .and_then(|l| l.split_once('='))
        .map(|(_, v)| v.trim_matches('"').to_owned())
        .context("Failed to parse pretty name")
}

pub fn all() -> Result<[String; 4]> {
    Ok([
        format!("\x1b[31;1m{}", title()),
        format!(
            "\x1b[36;1mos  {}",
            distro()
                .map_err(|e| eprintln!("Failed to get distro: {e}"))
                .unwrap_or("unknown".to_owned())
        ),
        format!(
            "\x1b[36;1mkr  {}",
            kernel()
                .map_err(|e| eprintln!("Failed to get kernel: {e}"))
                .unwrap_or("unknown".to_owned())
        ),
        format!("\x1b[36;1msh  {}", shell().unwrap_or("unknown".to_owned())),
    ])
}
