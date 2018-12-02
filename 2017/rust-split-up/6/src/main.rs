extern crate aoc_util;
use std::collections::HashSet;

fn main() {
    let numbers = aoc_util::get_input(6)
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();

    let cycles1 = get_cycles(numbers);
    println!("Part 1 {} Part 2 {}", cycles1.0, get_cycles(cycles1.1).0);
}

fn get_cycles(input: Vec<usize>) -> (usize, Vec<usize>) {
    let mut found_patterns = HashSet::new();
    let mut cycle_count = 0;
    let mut pattern = input;
    while found_patterns.insert(pattern.clone()) {
        pattern = rotate_numers(pattern);
        cycle_count += 1;
    }
    return (cycle_count, pattern);
}

fn rotate_numers(input: Vec<usize>) -> Vec<usize> {
    let mut cloned_input = input.clone();
    let mut highest_number = 0;
    let mut highest_index = 0;
    for i in 0..input.len() {
        if input[i] > highest_number {
            highest_index = i;
            highest_number = input[i]
        }
    }

    let to_add = highest_number / input.len();
    let remainder = highest_number % input.len();

    /*
    println!("----");
    println!("before {:?}", cloned_input);
    println!(
        "Highest num is {} at index {}",
        highest_number,
        highest_index
    );
    println!(
        "remainder {} to add {} for array {}",
        remainder,
        to_add,
        input_length
    );*/

    cloned_input[highest_index] = 0;
    for i in 0..input.len() {
        cloned_input[i] += to_add;
        if (input.len() + i - highest_index - 1) % input.len() < remainder {
            cloned_input[i] += 1;
        }
    }
    /*
    println!("after  {:?}", cloned_input);
    */

    return cloned_input;
}

#[cfg(test)]
mod test {
    use super::get_cycles_part_1;
    use super::rotate_numers;

    #[test]
    fn test_part_1() {
        let part1_example = get_cycles_part_1(vec![0, 2, 7, 0]);
        assert_eq!(part1_example, 5);
    }

    #[test]
    fn test1() {
        assert_eq!(rotate_numers(vec![0, 2, 7, 0]), vec![2, 4, 1, 2]);
    }

    #[test]
    fn test2() {
        assert_eq!(rotate_numers(vec![2, 4, 1, 2]), vec![3, 1, 2, 3]);
    }

    #[test]
    fn test3() {
        assert_eq!(rotate_numers(vec![3, 1, 2, 3]), vec![0, 2, 3, 4]);
    }

    #[test]
    fn test4() {
        assert_eq!(rotate_numers(vec![0, 2, 3, 4]), vec![1, 3, 4, 1]);
    }

    #[test]
    fn test5() {
        assert_eq!(rotate_numers(vec![1, 3, 4, 1]), vec![2, 4, 1, 2]);
    }
}
