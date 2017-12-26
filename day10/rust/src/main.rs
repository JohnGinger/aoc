use std::fs::File;
use std::io::Read;
extern crate ascii;
use ascii::AsciiChar;

fn main() {
    let file_name = "../input.txt";
    let mut file = File::open(file_name).expect("Unable to open input file!");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Cannot convert file contents to string!");

    let input_lengths = contents
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();

    let mut elements = Vec::with_capacity(256);
    for i in 0..256 {
        elements.push(i);
    }
    let hash_elements = hash_round(&input_lengths, 0, 0, elements).0;
    println!("Part 1 is {:?}", hash_elements[0] * hash_elements[1]);
    println!("Part 2 is {}", knot_hash(contents));
}

fn knot_hash(input: String) -> String {
    let mut lengths = get_ascii_string(input);
    lengths.append(&mut vec![17, 31, 73, 47, 23]);

    let mut position = 0;
    let mut skip = 0;
    let mut elements: Vec<usize> = (0..255).collect();
    elements.push(255);
    for _ in 0..64 {
        let round_result = hash_round(&lengths, position, skip, elements);
        elements = round_result.0;
        position = round_result.1;
        skip = round_result.2;
    }

    elements
        .chunks(16)
        .map(|chunk| chunk.iter().fold(0, |acc, v| acc ^ v))
        .map(|chunk| format!("{:02x}", chunk))
        .collect::<String>()
}

fn get_ascii_string(input: String) -> Vec<usize> {
    input
        .chars()
        .map(|x| AsciiChar::from(x).unwrap().as_byte() as usize)
        .collect()
}

fn hash_round(
    lengths: &Vec<usize>,
    mut position: usize,
    mut skip: usize,
    mut elements: Vec<usize>,
) -> (Vec<usize>, usize, usize) {
    let elements_length = elements.len();
    for &length in lengths {
        elements = rotate(elements, position);
        let mut end = elements.split_off(length);
        elements.reverse();
        elements.append(&mut end);
        elements = rotate(elements, elements_length - position);
        position = (position + length + skip) % elements_length;
        skip += 1;
    }

    return (elements, position, skip);
}

fn rotate(vec: Vec<usize>, mid: usize) -> Vec<usize> {
    let mut rotated = Vec::with_capacity(vec.len());
    for i in 0..vec.len() {
        rotated.push(vec[(i + mid) % vec.len()]);
    }
    return rotated;
}



#[cfg(test)]
mod test {
    use super::hash_round;
    use super::rotate;
    use super::get_ascii_string;
    use super::knot_hash;

    #[test]
    fn test_rotate() {
        let part1_example = rotate(vec![1, 2, 3, 4, 5], 2);
        assert_eq!(part1_example, vec![3, 4, 5, 1, 2]);
    }

    #[test]
    fn test_hash() {
        let elements = hash_round(&vec![3, 4, 1, 5], 0, 0, vec![0, 1, 2, 3, 4]).0;
        assert_eq!(elements[0] * elements[1], 12);
    }

    #[test]
    fn test_ascii() {
        let res = get_ascii_string(String::from("1,2,3"));
        assert_eq!(res, vec![49, 44, 50, 44, 51]);
    }

    #[test]
    fn test_knot1() {
        let res = knot_hash(String::from(""));
        assert_eq!(res, String::from("a2582a3a0e66e6e86e3812dcb672a272"));
    }

    #[test]
    fn test_knot2() {
        let res = knot_hash(String::from("AoC 2017"));
        assert_eq!(res, String::from("33efeb34ea91902bb2f59c9920caa6cd"));
    }

    #[test]
    fn test_knot3() {
        let res = knot_hash(String::from("1,2,3"));
        assert_eq!(res, String::from("3efbe78a8d82f29979031a4aa0b16a9d"));
    }

    #[test]
    fn test_knot4() {
        let res = knot_hash(String::from("1,2,4"));
        assert_eq!(res, String::from("63960835bcdc130f0b66d7ff4f6a5a8e"));
    }
}
