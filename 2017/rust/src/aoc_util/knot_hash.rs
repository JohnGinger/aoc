use ascii::AsciiChar;

pub fn get(input: &str) -> String {
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

fn get_ascii_string(input: &str) -> Vec<usize> {
    input
        .chars()
        .map(|x| AsciiChar::from(x).unwrap().as_byte() as usize)
        .collect()
}

pub fn hash_round(
    lengths: &[usize],
    mut position: usize,
    mut skip: usize,
    mut elements: Vec<usize>,
) -> (Vec<usize>, usize, usize) {
    let elements_length = elements.len();
    for &length in lengths {
        elements = rotate(&elements, position);
        let mut end = elements.split_off(length);
        elements.reverse();
        elements.append(&mut end);
        elements = rotate(&elements, elements_length - position);
        position = (position + length + skip) % elements_length;
        skip += 1;
    }

    (elements, position, skip)
}

fn rotate(vec: &[usize], mid: usize) -> Vec<usize> {
    let mut rotated = Vec::with_capacity(vec.len());
    for i in 0..vec.len() {
        rotated.push(vec[(i + mid) % vec.len()]);
    }
    rotated
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rotate() {
        let part1_example = rotate(&[1, 2, 3, 4, 5], 2);
        assert_eq!(part1_example, vec![3, 4, 5, 1, 2]);
    }

    #[test]
    fn test_hash() {
        let elements = hash_round(&[3, 4, 1, 5], 0, 0, vec![0, 1, 2, 3, 4]).0;
        assert_eq!(elements[0] * elements[1], 12);
    }

    #[test]
    fn test_ascii() {
        let res = get_ascii_string("1,2,3");
        assert_eq!(res, vec![49, 44, 50, 44, 51]);
    }

    #[test]
    fn test_knot1() {
        let res = get("");
        assert_eq!(res, String::from("a2582a3a0e66e6e86e3812dcb672a272"));
    }

    #[test]
    fn test_knot2() {
        let res = get("AoC 2017");
        assert_eq!(res, String::from("33efeb34ea91902bb2f59c9920caa6cd"));
    }

    #[test]
    fn test_knot3() {
        let res = get("1,2,3");
        assert_eq!(res, String::from("3efbe78a8d82f29979031a4aa0b16a9d"));
    }

    #[test]
    fn test_knot4() {
        let res = get("1,2,4");
        assert_eq!(res, String::from("63960835bcdc130f0b66d7ff4f6a5a8e"));
    }
}
