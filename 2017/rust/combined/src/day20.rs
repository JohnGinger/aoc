use crate::aoc_util;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::f32;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct ThreeD {
    x: isize,
    y: isize,
    z: isize,
}

impl ThreeD {
    fn len(&self) -> f32 {
        ((self.x * self.x) as f32 + (self.y * self.y) as f32 + (self.z * self.z) as f32).sqrt()
    }
}

fn build_three_d(input: &str) -> ThreeD {
    let numbers = input
        .split(',')
        .map(|x| {
            x.trim()
                .parse()
                .unwrap_or_else(|_| panic!("The regex went wrong {}", input))
        })
        .collect::<Vec<isize>>();
    ThreeD {
        x: numbers[0],
        y: numbers[1],
        z: numbers[2],
    }
}

#[derive(Debug)]
struct Instruction {
    position: ThreeD,
    velocity: ThreeD,
    acceleration: ThreeD,
}

impl Instruction {
    fn simulate(&mut self) {
        self.velocity.x += self.acceleration.x;
        self.velocity.y += self.acceleration.y;
        self.velocity.z += self.acceleration.z;

        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }
}

pub fn run() {
    let contents = aoc_util::get_input(20);

    let instructions = parse_instructions(&contents);
    let num_instructions = instructions.len();

    println!(
        "Part 1 is {:?}",
        instructions
            .iter()
            .map(|instruction| instruction.acceleration.len())
            .enumerate()
            .fold((0, f32::MAX), |min, (i, current)| if current < min.1 {
                (i, current)
            } else {
                min
            })
    );

    let mut removed = HashSet::new();
    let mut simulated_instructions = instructions;
    for _ in 1..1000 {
        let mut visited = HashMap::new();
        for (i, instruction) in simulated_instructions.iter_mut().enumerate() {
            if removed.contains(&i) {
                continue;
            }
            if let Some(x) = visited.insert(instruction.position, i) {
                removed.insert(i);
                removed.insert(x);
            }
            instruction.simulate();
        }
    }
    println!("Part 2 is {:?}", num_instructions - removed.len())
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"<(?P<p>[ 0-9,-]+)>.+<(?P<v>[ 0-9,-]+)>.+<(?P<a>[ 0-9,-]+)>").unwrap();
    }
    input
        .lines()
        .map(|line| RE.captures(line).expect("Line was badly formatted"))
        .map(|cap| Instruction {
            position: build_three_d(&cap["p"]),
            velocity: build_three_d(&cap["v"]),
            acceleration: build_three_d(&cap["a"]),
        })
        .collect()
}
