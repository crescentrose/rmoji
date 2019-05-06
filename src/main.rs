extern crate regex;

use std::env;
use std::fs;
use std::io::BufReader;

mod parser;

fn main() -> Result<(), std::io::Error>{
    let _args: Vec<String> = env::args().collect();

    const DATA_FILENAME: &str = "data/emoji-test.txt";
    let data_file = match fs::File::open(DATA_FILENAME) {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let reader = BufReader::new(data_file);
    let emojis = parser::read_emoji_file(reader);

    for emoji in emojis {
        println!("{} [{}] [{}] [{}]", emoji.description, emoji.group, emoji.subgroup, emoji.codepoint);
    }

    Ok(())
}

