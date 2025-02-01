mod info;

const CAT: [&str; 4] =
[
"\x1b[37m  ╱|、     ",
"\x1b[37m (˚ˎ。7    ",
"\x1b[37m  |、˜〵   ",
"\x1b[37m  じしˍ,)ノ",
];

fn main() {
    CAT.iter().zip(info::all().expect(":(")).for_each(|(c, i)| {
        println!("{c}  {i}\x1b[0m")
    });
}
