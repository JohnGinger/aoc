extern crate ascii;
use self::ascii::AsciiChar;

pub fn get(input: String) -> String {
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
