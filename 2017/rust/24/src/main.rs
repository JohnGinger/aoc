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

    println!("Part 1 is {}", get_chain_strongest(Vec::new(), &parts, 0));
    println!(
        "Part 2 is {}",
        get_chain_longest(Vec::new(), &parts, 0).strength
    );
}

#[derive(Clone, Debug, Copy)]
struct Part {
    a: usize,
    b: usize,
}

fn get_chain_strongest(chain: Vec<Part>, remaining_parts: &Vec<Part>, tail: usize) -> usize {
    match remaining_parts
        .into_iter()
        .enumerate()
        .map(|(index, part)| {
            if part.a == tail || part.b == tail {
                let mut new_remaining_parts = remaining_parts.clone();
                let part_to_add = new_remaining_parts.remove(index);
                let mut new_chain = chain.clone();
                new_chain.push(part_to_add);
                if part.a == tail {
                    return get_chain_strongest(new_chain, &new_remaining_parts, part.b);
                } else {
                    return get_chain_strongest(new_chain, &new_remaining_parts, part.a);
                }
            } else {
                return BestChain::from_chain(&chain).strength;
            }
        })
        .max()
    {
        Some(strength) => strength,
        None => BestChain::from_chain(&chain).strength,
    }
}

fn get_chain_longest(chain: Vec<Part>, remaining_parts: &Vec<Part>, tail: usize) -> BestChain {
    let candidates = remaining_parts
        .into_iter()
        .enumerate()
        .map(|(index, part)| {
            if part.a == tail || part.b == tail {
                let mut new_remaining_parts = remaining_parts.clone();
                let part_to_add = new_remaining_parts.remove(index);
                let mut new_chain = chain.clone();
                new_chain.push(part_to_add);
                if part.a == tail {
                    return get_chain_longest(new_chain, &new_remaining_parts, part.b);
                } else {
                    return get_chain_longest(new_chain, &new_remaining_parts, part.a);
                }
            } else {
                return BestChain::from_chain(&chain);
            }
        })
        .collect::<Vec<BestChain>>();

    let mut best = BestChain::from_chain(&chain);
    for candidate in candidates {
        if candidate.length > best.length {
            best = candidate
        } else if candidate.length == best.length && candidate.strength > best.strength {
            best = candidate
        }
    }
    return best;
}

struct BestChain {
    length: usize,
    strength: usize,
}

impl BestChain {
    fn from_chain(chain: &Vec<Part>) -> BestChain {
        return BestChain {
            length: chain.len(),
            strength: chain.into_iter().map(|part| part.a + part.b).sum(),
        };
    }
}
