use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Awesome list from HIT!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

// --------------------------
// | Awesome list from HIT! |
// --------------------------
//               \
//                \
//                   _~^~^~_
//               \) /  o o  \ (/
//                 '_   -   _'
//                 / '-----' \
