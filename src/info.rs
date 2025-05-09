use anyhow::{Context, Result};
use std::{env::var, fs::read_to_string, process::Command};

fn get_user() -> Result<String> {
    if let Ok(user) = var("USER") {
        Ok(user)
    } else {
        Ok(String::from_utf8_lossy(
            Command::new("whoami")
                .output()
                .context("Couldn't detect user")?
                .stdout
                .trim_ascii(),
        )
        .to_string())
    }
}

fn title() -> Result<String> {
    Ok(get_user()?
        + "@"
        + read_to_string("/etc/hostname")
            .context("Couldn't read /etc/hostname")?
            .trim())
}

fn kernel() -> Result<String> {
    Ok(read_to_string("/proc/version")
        .context("Couldn't read /proc/version")?
        .split_whitespace()
        .nth(2)
        .context("Failed to get kernel version")?
        .to_string())
}

fn shell() -> Result<String> {
    let shell = if let Ok(shell) = var("SHELL") {
        shell
    } else {
        String::from_utf8_lossy(
            Command::new("readlink")
                .arg("/bin/sh")
                .output()
                .expect("Couldn't detect shell")
                .stdout
                .trim_ascii(),
        )
        .to_string()
    };

    Ok(shell
        .rsplit_once('/')
        .map(|(_, sh)| sh.to_string())
        .unwrap_or(shell))
}

fn distro() -> Result<String> {
    read_to_string("/etc/os-release")
        .context("Couldn't read /etc/os-release")?
        .lines()
        .find(|l| l.starts_with("PRETTY_NAME="))
        .and_then(|l| l.split_once('='))
        .map(|(_, v)| v.trim_matches('"').to_string())
        .context("Failed to parse pretty name")
}

pub fn all() -> Result<[String; 4]> {
    Ok([
        format!("\x1b[31;1m{}", title()?),
        format!("\x1b[36;1mos  {}", distro()?),
        format!("\x1b[36;1mkr  {}", kernel()?),
        format!("\x1b[36;1msh  {}", shell()?),
    ])
}
