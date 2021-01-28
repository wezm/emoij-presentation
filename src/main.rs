use std::env;

mod data;

fn main() {
    for arg in env::args().skip(1) {
        for c in arg.chars() {
            println!(
                "{} Emoji={}, Emoji_Presentation={}",
                c,
                data::EMOJI.contains_char(c),
                data::EMOJI_PRESENTATION.contains_char(c)
            );
        }
    }
}
