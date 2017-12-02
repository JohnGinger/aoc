use std::fs::File;
use std::io::Read;

fn main() {
    let file_name = "../input.txt";
    let mut file = File::open(file_name).expect("Unable to open input file!");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect(
        "Cannot convert file contents to string!",
    );

    let lines: Vec<&str> = contents
        .trim()
        .split("\n")
        .filter(|s| !s.is_empty())
        .collect();

    let mut part1 = 0;
    let mut part2 = 0;
    for line in lines {
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
                    println!("Even divide {}", even_divide)
                }
                if numbers[check_index] % *num == 0 {
                    even_divide = numbers[check_index] / *num;
                    println!("Even divide {}", even_divide)
                }
            }
        }
        part1 += max - min;
        part2 += even_divide
    }
    println!("Part 1 is {}", part1);
    println!("Part 1 is {}", part2);
}