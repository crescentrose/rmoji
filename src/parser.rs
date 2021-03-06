use std::convert::TryFrom;
use std::io::{BufRead, BufReader};
use regex::Regex;
use std::fs;

pub struct Emoji {
    pub codepoint: String,
    pub description: String,
    pub group: String,
    pub subgroup: String,
}

pub fn read_emoji_file(reader: BufReader<fs::File>) -> Vec<Emoji> {
    let mut emojis: Vec<Emoji> = Vec::new();

    let emoji_regex = Regex::new(r"^([0-9A-F\s]*)\s+;\s*fully-qualified\s*#\s?(.*)$")
        .unwrap();
    let group_regex = Regex::new(r"^#\sgroup:\s(.*)$")
        .unwrap();
    let subgroup_regex = Regex::new(r"^#\ssubgroup:\s(.*)$")
        .unwrap();

    let mut group = String::from("default");
    let mut subgroup = String::from("default");

    for line in reader.lines() {
        let line = line.unwrap();
        if emoji_regex.is_match(&line) {
            let matches = emoji_regex.captures(&line).unwrap();
            emojis.push(Emoji {
                codepoint: codepoint_from_string(matches.get(1).unwrap().as_str()),
                description: String::from(matches.get(2).unwrap().as_str()),
                group: group.clone(),
                subgroup: subgroup.clone(),
            });
        } else if group_regex.is_match(&line) {
            group = group_regex.captures(&line).unwrap().get(1).unwrap().as_str().to_string();
        } else if subgroup_regex.is_match(&line) {
            subgroup = subgroup_regex.captures(&line).unwrap().get(1).unwrap().as_str().to_string();
        }
    }

    emojis
}

// TODO: this doubled the execution time, see if it can be optimized somehow
fn codepoint_from_string(string: &str) -> String {
    let mut emoji: Vec<char> = Vec::new();

    for codepoint in string.trim().split(' ') {
        emoji.push(
            char::try_from(
                u32::from_str_radix(&codepoint, 16)
                .expect(&format!("{} should have been a hex string", codepoint))
                ).unwrap()
            );
    }

    emoji.into_iter().collect()
}
