extern crate aoc_util;
use std::collections::HashSet;

#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::iter::FromIterator;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Disc {
    id: String,
    weight: i32,
    holding: Vec<String>,
}

fn main() {
    let disks = aoc_util::iterate_input_lines(7)
        .map(|x| get_disk_from_line(&x))
        .collect::<Vec<Disc>>();
    let disks_to_remove: HashSet<String> = HashSet::from_iter(
        disks
            .iter()
            .cloned()
            .filter(|x| x.holding.len() > 0)
            .flat_map(|x| x.holding),
    );

    let bottom_disk = disks
        .iter()
        .cloned()
        .filter(|x| !disks_to_remove.contains(&x.id))
        .collect::<Vec<Disc>>();

    let part1_id = String::from(bottom_disk[0].clone().id);
    println!("Part 1 {}", part1_id);
    get_weight(part1_id, &disks);
}

fn get_disk_from_line(line: &str) -> Disc {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?P<id>[a-z]+) \((?P<weight>[0-9]+)\)( -> (?P<holding>[a-z, ]+))?")
                .unwrap();
    }

    match RE.captures(line) {
        Some(cap) => {
            let holding = &cap.name("holding");
            if holding.is_none() {
                return Disc {
                    id: String::from(&cap["id"]),
                    weight: (&cap["weight"]).parse().unwrap(),
                    holding: vec![],
                };
            } else {
                return Disc {
                    id: String::from(&cap["id"]),
                    weight: (&cap["weight"]).parse().unwrap(),
                    holding: holding
                        .unwrap()
                        .as_str()
                        .split(",")
                        .map(|x| String::from(x.trim()))
                        .collect::<Vec<String>>(),
                };
            }
        }
        None => Disc {
            id: String::from(""),
            weight: 0,
            holding: vec![],
        },
    }
}

fn get_disk(id: String, disks: &Vec<Disc>) -> Option<&Disc> {
    return disks.iter().find(|disk| disk.id == id);
}

fn get_weight(id: String, disks: &Vec<Disc>) -> i32 {
    let disk = get_disk(id.clone(), disks).unwrap();
    if disk.holding.len() > 0 {
        let weights = disk
            .holding
            .iter()
            .map(|x| get_weight(String::from(x.clone()), disks))
            .collect::<Vec<i32>>();

        for i in 1..weights.len() {
            if weights[i] != weights[i - 1] {
                let uneven_disk_id = disk.holding[i].clone();
                let uneven_disk = get_disk(uneven_disk_id, disks).unwrap();
                println!(
                    "Part 2 new disk weight {}",
                    uneven_disk.weight - (weights[i] - weights[i - 1])
                );
                break;
            }
        }
        let weights_sum: i32 = weights.iter().sum();
        return disk.weight + weights_sum;
    } else {
        return disk.weight;
    }
}
