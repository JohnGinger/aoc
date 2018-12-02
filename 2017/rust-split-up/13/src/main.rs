extern crate aoc_util;

#[derive(Debug, Clone)]
struct FirewallLevel {
    layer: usize,
    depth: usize,
}

fn main() {
    let mut levels = Vec::new();
    for line in aoc_util::iterate_input_lines(13) {
        let values = line
            .split(":")
            .map(|x| x.trim().parse().unwrap())
            .collect::<Vec<usize>>();
        levels.push(FirewallLevel {
            layer: values[0],
            depth: values[1],
        });
    }
    println!("Part 1 is {}", get_severity(get_caught_levels(&levels, 0)));

    let mut delay = 0;
    loop {
        if get_caught_levels(&levels, delay).len() == 0 {
            break;
        }
        delay += 1;
    }
    println!("Part 2 is {}", delay);
}

fn is_caught(delay: usize, depth: usize, layer: usize) -> bool {
    (layer + delay) % (2 * (depth - 1)) == 0
}

fn get_caught_levels(levels: &Vec<FirewallLevel>, delay: usize) -> Vec<FirewallLevel> {
    levels
        .into_iter()
        .filter(|level| is_caught(delay, level.depth, level.layer))
        .cloned()
        .collect()
}

fn get_severity(levels: Vec<FirewallLevel>) -> usize {
    levels
        .into_iter()
        .fold(0, |severity, level| severity + level.depth * level.layer)
}
