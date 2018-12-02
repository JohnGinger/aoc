use crate::aoc_util::get_input;

pub fn run() {
    let numbers: Vec<i32> = get_input(1)
        .split("")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();
    let mut total1 = 0;
    let mut total2 = 0;
    for i in 1..numbers.len() {
        let part1_index = (i + 1) % numbers.len();
        let part2_index = (i + numbers.len() / 2) % numbers.len();

        if numbers[part1_index] == numbers[i] {
            total1 += numbers[part1_index];
        }

        if numbers[part2_index] == numbers[i] {
            total2 += numbers[part2_index];
        }
    }

    println!("Part 1: {}", total1);
    println!("Part 2: {}", total2);
}
