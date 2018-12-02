use crate::aoc_util;
use std::collections::HashSet;

pub fn run() {
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
        pattern = rotate_numers(&pattern);
        cycle_count += 1;
    }
    (cycle_count, pattern)
}

fn rotate_numers(input: &[usize]) -> Vec<usize> {
    let mut cloned_input = input.to_owned();
    let mut highest_number = 0;
    let mut highest_index = 0;
    for (i, &number) in input.iter().enumerate() {
        if number > highest_number {
            highest_index = i;
            highest_number = number
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
    for (i, cloned_input_element) in cloned_input.iter_mut().enumerate() {
        *cloned_input_element += to_add;
        if (input.len() + i - highest_index - 1) % input.len() < remainder {
            *cloned_input_element += 1;
        }
    }
    /*
    println!("after  {:?}", cloned_input);
    */

    cloned_input
}

#[cfg(test)]
mod test {
    use super::get_cycles;
    use super::rotate_numers;

    #[test]
    fn test_part_1() {
        let part1_example = get_cycles(vec![0, 2, 7, 0]);
        assert_eq!(part1_example.0, 5);
    }

    #[test]
    fn test1() {
        assert_eq!(rotate_numers(&[0, 2, 7, 0]), vec![2, 4, 1, 2]);
    }

    #[test]
    fn test2() {
        assert_eq!(rotate_numers(&[2, 4, 1, 2]), vec![3, 1, 2, 3]);
    }

    #[test]
    fn test3() {
        assert_eq!(rotate_numers(&[3, 1, 2, 3]), vec![0, 2, 3, 4]);
    }

    #[test]
    fn test4() {
        assert_eq!(rotate_numers(&[0, 2, 3, 4]), vec![1, 3, 4, 1]);
    }

    #[test]
    fn test5() {
        assert_eq!(rotate_numers(&[1, 3, 4, 1]), vec![2, 4, 1, 2]);
    }
}
