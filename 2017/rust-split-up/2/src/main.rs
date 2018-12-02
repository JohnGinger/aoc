extern crate aoc_util;

fn main() {
    let mut part1 = 0;
    let mut part2 = 0;
    for line in aoc_util::iterate_input_lines(2) {
        let numbers: Vec<i32> = line.split_whitespace()
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
            for check_index in (index + 1)..numbers.len() {
                if *num % numbers[check_index] == 0 {
                    even_divide = *num / numbers[check_index];
                }
                if numbers[check_index] % *num == 0 {
                    even_divide = numbers[check_index] / *num;
                }
            }
        }
        part1 += max - min;
        part2 += even_divide
    }
    println!("Part 1 is {}", part1);
    println!("Part 2 is {}", part2);
}
