use crate::aoc_util;

pub fn run() {
    let mut part1 = 0;
    let mut part2 = 0;
    for line in aoc_util::iterate_input_lines(2) {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let mut min = i32::max_value();
        let mut max = 0;
        let mut even_divide = 0;
        for (index, num) in numbers.iter().enumerate() {
            if *num > max {
                max = *num;
            }
            if *num < min {
                min = *num;
            }
            for num_to_check in numbers.iter().skip(index + 1) {
                if *num % num_to_check == 0 {
                    even_divide = *num / num_to_check;
                }
                if num_to_check % *num == 0 {
                    even_divide = num_to_check / *num;
                }
            }
        }
        part1 += max - min;
        part2 += even_divide
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
