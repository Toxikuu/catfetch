mod info;

use std::env::args;
use std::process::exit;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

const CAT: [&str; 4] = [
    "\x1b[37m  ╱|、     ",
    "\x1b[37m (˚ˎ。7    ",
    "\x1b[37m  |、˜〵   ",
    "\x1b[37m  じしˍ,)ノ",
];

fn main() {
    show_version();

    let Ok(info) = info::all().map_err(|e| eprintln!("Error: {e}")) else {
        exit(1)
    };

    CAT.iter()
        .zip(info)
        .for_each(|(c, i)| println!("{c}  {i}\x1b[0m"));
}

fn show_version() {
    if args().any(|a| matches!(a.as_str(), "-V" | "--version")) {
        println!("{NAME} {VERSION}");
        exit(0)
    }
}
