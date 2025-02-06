use anyhow::{Context, Result};
use std::{
    fs::read_to_string,
    env::var,
};

fn title() -> Result<String> {
    Ok(
        var("USER")? + "@" + read_to_string("/etc/hostname")?.trim()
    )
}

fn kernel() -> Result<String> {
    Ok(
        read_to_string("/proc/version")?
            .split_whitespace()
            .nth(2)
            .context("Failed to get kernel version")?
            .to_string()
    )
}

fn shell() -> Result<String> {
    var("SHELL")?
        .rsplit_once('/')
        .map(|(_, sh)| sh.to_string())
        .context("Invalid $SHELL")
}

fn distro() -> Result<String> {
    read_to_string("/etc/os-release")?
        .lines()
        .find(|l| l.starts_with("PRETTY_NAME="))
        .and_then(|l| l.split_once('='))
        .map(|(_, v)| v.trim_matches('"').to_string())
        .context("Failed to parse pretty name")
}

pub fn all() -> Result<[String; 4]> {
    Ok([
        format!("\x1b[31;1m{}"    , title()?),
        format!("\x1b[36;1mos  {}", distro()?),
        format!("\x1b[36;1mkr  {}", kernel()?),
        format!("\x1b[36;1msh  {}", shell()?),
    ])
}
