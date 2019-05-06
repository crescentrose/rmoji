extern crate regex;

mod parser;
mod errors;

use std::fs;
use std::io::BufReader;
use errors::*;

fn run() -> Result<(), errors::RmojiError> {
    const DATA_FILENAME: &str = "data/emoji-test.txt";
    let data_file = match fs::File::open(DATA_FILENAME) {
        Ok(file) => file,
        Err(_e) => return Err(RmojiError::new("unreadable data file"))
    };

    let reader = BufReader::new(data_file);
    let emojis = parser::read_emoji_file(reader);

    for emoji in emojis {
        println!("{} [{}] [{}] [{}]", emoji.description, emoji.group, emoji.subgroup, emoji.codepoint);
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("error: {}", e);

        std::process::exit(1)
    }
}

