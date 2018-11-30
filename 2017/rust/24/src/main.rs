extern crate aoc_util;

fn main() {
    let lines = aoc_util::iterate_input_lines(24);

    let parts = lines
        .map(|line| {
            let sides = line
                .trim()
                .split("/")
                .map(|x| x.parse().expect("Should be a number"))
                .collect::<Vec<usize>>();
            Part {
                a: sides[0],
                b: sides[1],
            }
        })
        .collect::<Vec<Part>>();

    println!("Part 1 is {}", get_strongest_bridge(Vec::new(), &parts, 0));
    println!(
        "Part 2 is {}",
        get_longest_bridge(Vec::new(), &parts, 0).strength
    );
}

#[derive(Clone, Debug, Copy)]
struct Part {
    a: usize,
    b: usize,
}

fn get_strongest_bridge(bridge: Vec<Part>, remaining_parts: &Vec<Part>, tail: usize) -> usize {
    match remaining_parts
        .into_iter()
        .enumerate()
        .map(|(index, part)| {
            if part.a == tail || part.b == tail {
                let mut new_remaining_parts = remaining_parts.clone();
                let part_to_add = new_remaining_parts.remove(index);
                let mut new_bridge = bridge.clone();
                new_bridge.push(part_to_add);
                if part.a == tail {
                    return get_strongest_bridge(new_bridge, &new_remaining_parts, part.b);
                } else {
                    return get_strongest_bridge(new_bridge, &new_remaining_parts, part.a);
                }
            } else {
                return BridgeProperties::from_bridge(&bridge).strength;
            }
        })
        .max()
    {
        Some(strength) => strength,
        None => BridgeProperties::from_bridge(&bridge).strength,
    }
}

fn get_longest_bridge(
    bridge: Vec<Part>,
    remaining_parts: &Vec<Part>,
    tail: usize,
) -> BridgeProperties {
    let candidates = remaining_parts
        .into_iter()
        .enumerate()
        .map(|(index, part)| {
            if part.a == tail || part.b == tail {
                let mut new_remaining_parts = remaining_parts.clone();
                let part_to_add = new_remaining_parts.remove(index);
                let mut new_bridge = bridge.clone();
                new_bridge.push(part_to_add);
                if part.a == tail {
                    return get_longest_bridge(new_bridge, &new_remaining_parts, part.b);
                } else {
                    return get_longest_bridge(new_bridge, &new_remaining_parts, part.a);
                }
            } else {
                return BridgeProperties::from_bridge(&bridge);
            }
        })
        .collect::<Vec<BridgeProperties>>();

    let mut best = BridgeProperties::from_bridge(&bridge);
    for candidate in candidates {
        if candidate.length > best.length {
            best = candidate
        } else if candidate.length == best.length && candidate.strength > best.strength {
            best = candidate
        }
    }
    return best;
}

struct BridgeProperties {
    length: usize,
    strength: usize,
}

impl BridgeProperties {
    fn from_bridge(bridge: &Vec<Part>) -> BridgeProperties {
        return BridgeProperties {
            length: bridge.len(),
            strength: bridge.into_iter().map(|part| part.a + part.b).sum(),
        };
    }
}
