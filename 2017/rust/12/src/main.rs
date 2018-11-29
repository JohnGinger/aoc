extern crate aoc_util;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let mut groups = HashMap::new();
    let mut location_counter: usize = 0;
    for line in aoc_util::iterate_input_lines(12) {
        groups.insert(location_counter, get_connected(&line));
        location_counter += 1;
    }

    let mut seen_so_far = HashSet::new();
    get_children(0, &groups, &mut seen_so_far);
    println!("Part 1 is {}", seen_so_far.len());

    let mut total_seen: HashSet<usize> = HashSet::new();
    let mut groups_count = 0;
    let locations = (0..location_counter).collect::<Vec<_>>();
    loop {
        let group_start = locations.iter().find(|x| !total_seen.contains(x));
        match group_start {
            Some(&start) => get_children(start, &groups, &mut total_seen),
            None => break,
        };
        groups_count += 1;
    }

    println!("Part 2 is {}", groups_count);
}

fn get_children(
    address: usize,
    groups: &HashMap<usize, Vec<usize>>,
    seen_so_far: &mut HashSet<usize>,
) -> Vec<usize> {
    match groups.get(&address) {
        Some(addresses) => {
            seen_so_far.insert(address);
            let seen_so_far_clone = seen_so_far.clone();
            addresses
                .iter()
                .filter(|x| !seen_so_far_clone.contains(x))
                .flat_map(|&child_address| get_children(child_address, groups, seen_so_far))
                .collect()
        }
        None => vec![],
    }
}

fn get_connected(line: &str) -> Vec<usize> {
    line.split(" <-> ")
        .nth(1)
        .unwrap()
        .split(",")
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect()
}
