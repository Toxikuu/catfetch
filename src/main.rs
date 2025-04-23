mod info;

const CAT: [&str; 4] = [
    "\x1b[37m  ╱|、     ",
    "\x1b[37m (˚ˎ。7    ",
    "\x1b[37m  |、˜〵   ",
    "\x1b[37m  じしˍ,)ノ",
];

#[cfg(feature = "error_handling")]
fn main() {
    use std::process::exit;

    let Ok(info) = info::all().map_err(|e| eprintln!("Error: {e}")) else {
        exit(1)
    };

    CAT.iter()
        .zip(info)
        .for_each(|(c, i)| println!("{c}  {i}\x1b[0m"));
}

#[cfg(not(feature = "error_handling"))]
fn main() {
    CAT.iter()
        .zip(info::all())
        .for_each(|(c, i)| println!("{c}  {i}\x1b[0m"));
}
