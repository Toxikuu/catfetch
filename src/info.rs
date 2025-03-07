#[cfg(feature = "error_handling")]
use anyhow::{Context, Result};
use std::{
    fs::read_to_string,
    env::var,
};

#[cfg(feature = "error_handling")]
fn title() -> Result<String> {
    Ok(
        var("USER")? + "@" + read_to_string("/etc/hostname")?.trim()
    )
}

#[cfg(not(feature = "error_handling"))]
fn title() -> String {
    var("USER").unwrap() + "@" + read_to_string("/etc/hostname").unwrap().trim()
}

#[cfg(feature = "error_handling")]
fn kernel() -> Result<String> {
    Ok(
        read_to_string("/proc/version")?
            .split_whitespace()
            .nth(2)
            .context("Failed to get kernel version")?
            .to_string()
    )
}

#[cfg(not(feature = "error_handling"))]
fn kernel() -> String {
    read_to_string("/proc/version")
        .unwrap()
        .split_whitespace()
        .nth(2)
        .unwrap()
        .to_string()
}

#[cfg(feature = "error_handling")]
fn shell() -> Result<String> {
    let shell = var("SHELL")?;
    Ok(
        shell
            .rsplit_once('/')
            .map(|(_, sh)| sh.to_string())
            .unwrap_or(shell)
    )
}

#[cfg(not(feature = "error_handling"))]
fn shell() -> String {
    let shell = var("SHELL").unwrap();
    shell
        .rsplit_once('/')
        .map(|(_, sh)| sh.to_string())
        .unwrap_or(shell)
}

#[cfg(feature = "error_handling")]
fn distro() -> Result<String> {
    read_to_string("/etc/os-release")?
        .lines()
        .find(|l| l.starts_with("PRETTY_NAME="))
        .and_then(|l| l.split_once('='))
        .map(|(_, v)| v.trim_matches('"').to_string())
        .context("Failed to parse pretty name")
}

#[cfg(not(feature = "error_handling"))]
fn distro() -> String {
    read_to_string("/etc/os-release")
        .unwrap()
        .lines()
        .find(|l| l.starts_with("PRETTY_NAME="))
        .and_then(|l| l.split_once('='))
        .map(|(_, v)| v.trim_matches('"').to_string())
        .unwrap()
}

#[cfg(feature = "error_handling")]
pub fn all() -> Result<[String; 4]> {
    Ok([
        format!("\x1b[31;1m{}"    , title()?),
        format!("\x1b[36;1mos  {}", distro()?),
        format!("\x1b[36;1mkr  {}", kernel()?),
        format!("\x1b[36;1msh  {}", shell()?),
    ])
}

#[cfg(not(feature = "error_handling"))]
pub fn all() -> [String; 4] {
    [
        format!("\x1b[31;1m{}"    , title()),
        format!("\x1b[36;1mos  {}", distro()),
        format!("\x1b[36;1mkr  {}", kernel()),
        format!("\x1b[36;1msh  {}", shell()),
    ]
}
