extern crate aoc_util;

fn main() {
    let input = aoc_util::get_input(9);
    let chars = input.split("").collect::<Vec<&str>>();

    let mut stack = Vec::new();
    let mut in_garbage = false;
    let mut ignore_next = false;
    let mut score1 = 0;
    let mut score2 = 0;

    for c in chars {
        if in_garbage {
            if ignore_next {
                ignore_next = false;
                continue;
            } else if c == "!" {
                ignore_next = true
            } else if c == ">" {
                in_garbage = false;
            } else {
                score2 += 1;
            }
        } else if c == "<" {
            in_garbage = true
        } else if c == "{" {
            stack.insert(0, String::from("{"))
        } else if c == "}" {
            if stack[0] == "{" {
                stack.remove(0);
                score1 += stack.len() + 1;
            }
        }
    }

    println!("Part 1 is {}", score1);
    println!("Part 2 is {}", score2);
}
