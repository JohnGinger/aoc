use crate::aoc_util;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn run() {
    let mut part1 = 0;
    let mut part2 = 0;
    for line in aoc_util::iterate_input_lines(4) {
        let mut words_set = HashSet::new();
        let mut sorted_words_set = HashSet::new();
        let mut valid_part1 = true;
        let mut valid_part2 = true;
        for word in line.split_whitespace() {
            if valid_part1 {
                valid_part1 = words_set.insert(word);
            }
            if valid_part2 {
                valid_part2 = sorted_words_set.insert(get_sorted(word));
            }
        }
        if valid_part1 {
            part1 += 1;
        }

        if valid_part2 {
            part2 += 1;
        }
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn get_sorted(word: &str) -> String {
    let mut characters = word.chars().collect::<Vec<char>>();
    characters.sort();
    String::from_iter(characters)
}
