use std::fs::File;
use std::io::Read;

fn main() {
    let file_name = "input.txt";
    let mut file = File::open(file_name).expect("Unable to open input file!");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect(
        "Cannot convert file contents to string!",
    );

    let numbers: Vec<i32> = contents
        .trim()
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

    println!("Part 1 solution: {}", total1);

    println!("Part 2 solution: {}", total2);
}