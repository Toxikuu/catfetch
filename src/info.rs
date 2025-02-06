use std::{
    fs::read_to_string,
    env::var,
};

fn title() -> String {
    var("USER").unwrap() + "@" + read_to_string("/etc/hostname").unwrap().trim()
}

fn kernel() -> String {
    read_to_string("/proc/version").unwrap()
        .split_whitespace()
        .nth(2)
        .unwrap()
        .to_string()
}

fn shell() -> String {
    var("SHELL").unwrap()
        .rsplit_once('/')
        .map(|(_, sh)| sh.to_string())
        .unwrap()
}

fn distro() -> String {
    read_to_string("/etc/os-release").unwrap()
        .lines()
        .find(|l| l.starts_with("PRETTY_NAME="))
        .and_then(|l| l.split_once('='))
        .map(|(_, v)| v.trim_matches('"').to_string())
        .unwrap()
}

pub fn all() -> [String; 4] {
    [
        format!("\x1b[31;1m{}"    , title ()),
        format!("\x1b[36;1mos  {}", distro()),
        format!("\x1b[36;1mkr  {}", kernel()),
        format!("\x1b[36;1msh  {}", shell ()),
    ]
}
