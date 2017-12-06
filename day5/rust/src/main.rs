use std::fs::File;
use std::io::Read;

fn main() {
    let file_name = "../input.txt";
    let mut file = File::open(file_name).expect("Unable to open input file!");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect(
        "Cannot convert file contents to string!",
    );

    let numbers = contents
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();


    println!("Part 1 {}", part1(&numbers));
    println!("Part 2 {}", part2(&numbers));
}

fn part1(input: &Vec<i32>) -> i32 {
    let mut position: i32 = 0;
    let mut count1 = 0;
    let mut numbers = input.clone();
    while position < numbers.len() as i32 && position >= 0 {
        let index = position as usize;
        let jump = numbers[index];
        numbers[index] += 1;
        position += jump;
        count1 += 1;
    }
    return count1;
}

fn part2(numbers: &Vec<i32>) -> i32 {
    let mut position = 0;
    let mut numbers2 = numbers.clone();
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
    return count2;
}

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