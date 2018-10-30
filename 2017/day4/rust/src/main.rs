use std::fs::File;
use std::io::Read;
use std::iter::FromIterator;
use std::collections::HashSet;

fn main() {
    let file_name = "../input.txt";
    let mut file = File::open(file_name).expect("Unable to open input file!");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect(
        "Cannot convert file contents to string!",
    );

    let mut part1 = 0;
    let mut part2 = 0;
    for line in contents.lines() {
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
    println!("Part 1 {} Part 2 {}", part1, part2);
}

fn get_sorted(word: &str) -> String {
    let mut characters = word.chars().collect::<Vec<char>>();
    characters.sort();
    return String::from_iter(characters);
}