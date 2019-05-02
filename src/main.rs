extern crate regex;

use std::env;
use std::fs;
use std::io::BufReader;

mod parser;

fn main() {
    let _args: Vec<String> = env::args().collect();

    const DATA_FILENAME: &str = "data/emoji-test.txt";
    let data_file = fs::File::open(DATA_FILENAME).unwrap();

    let reader = BufReader::new(data_file);
    let emojis = parser::read_emoji_file(reader);

    for emoji in emojis {
        println!("{} [{}] [{}] [{}]", emoji.description, emoji.group, emoji.subgroup, emoji.codepoint);
    }
}


