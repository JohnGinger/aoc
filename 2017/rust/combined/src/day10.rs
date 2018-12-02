use crate::aoc_util;
use crate::aoc_util::knot_hash;

pub fn run() {
    let contents = aoc_util::get_input(10);
    let input_lengths = contents
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();

    let mut elements = Vec::with_capacity(256);
    for i in 0..256 {
        elements.push(i);
    }
    let hash_elements = knot_hash::hash_round(&input_lengths, 0, 0, elements).0;
    println!("Part 1 is {:?}", hash_elements[0] * hash_elements[1]);
    println!("Part 2 is {}", knot_hash::get(&contents));
}
