use crate::aoc_util;

pub fn run() {
    let numbers = aoc_util::get_input(5)
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();

    println!("Part 1 {}", part1(&numbers));
    println!("Part 2 {}", part2(&numbers));
}

fn part1(input: &[i32]) -> i32 {
    let mut position: i32 = 0;
    let mut count1 = 0;
    let mut numbers = input.to_owned();
    while position < numbers.len() as i32 && position >= 0 {
        let index = position as usize;
        let jump = numbers[index];
        numbers[index] += 1;
        position += jump;
        count1 += 1;
    }
    count1
}

fn part2(numbers: &[i32]) -> i32 {
    let mut position = 0;
    let mut numbers2 = numbers.to_owned();
    let mut count2 = 0;
    while position < numbers2.len() as i32 && position >= 0 {
        let index = position as usize;
        let jump = numbers2[index];
        if jump >= 3 {
            numbers2[index] -= 1;
        } else {
            numbers2[index] += 1;
        }
        position += jump;
        count2 += 1;
    }
    count2
}
/*
#[cfg(test)]
mod test {
    use super::part1;

    use super::part2;

    #[test]
    fn test_part_1() {
        let part1_example = part1(vec![0, 3, 0, 1, -3]);
        assert_eq!(part1_example, 5);
    }
    #[test]
    fn test_part_2() {
        let part1_example = part2(vec![0, 3, 0, 1, -3]);
        assert_eq!(part1_example, 10);
    }
}
*/
