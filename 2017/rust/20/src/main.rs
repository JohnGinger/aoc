use std::fs::File;
use std::io::Read;
use std::collections::HashSet;
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;
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
        .split(",")
        .map(|x| {
            x.trim().parse().expect(&format!("The regex went wrong {}", input))
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

impl Instruction{
    fn simulate(&mut self) {
        self.velocity.x += self.acceleration.x;
        self.velocity.y += self.acceleration.y;
        self.velocity.z += self.acceleration.z;

        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }
}

fn main() {
    let file_name = "../input.txt";
    let mut file = File::open(file_name).expect("Unable to open input file!");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Cannot convert file contents to string!");

    let instructions = parse_instructions(contents);
    let num_instructions = instructions.len();
    //println!("{:?}", instructions.iter().map(|instruction| instruction.acceleration.len()).collect::<Vec<f32>>());

    println!("Part 1 is {:?}", instructions
    .iter()
    .map(|instruction| instruction.acceleration.len())
    .enumerate()
    .fold((0, f32::MAX), |min, (i, current)| if current < min.1 {
        (i, current)
    } else {
        min
    }));

    let mut removed = HashSet::new();
    let mut simulated_instructions = instructions;
    for _ in 1..1000 {
        let mut visited = HashMap::new();
        for i in 0..simulated_instructions.len() {
            let instruction = &mut simulated_instructions[i];
            if removed.contains(&i){
                continue;
            }
            match visited.insert(instruction.position, i) {
                Some(x) => {
                    removed.insert(i);
                    removed.insert(x);
                    ()
                },
                None => ()
            }
            instruction.simulate();
        }
    }
    println!("Part 2 is {:?}", num_instructions - removed.len())
}


fn parse_instructions(input: String) -> Vec<Instruction> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"<(?P<p>[ 0-9,-]+)>.+<(?P<v>[ 0-9,-]+)>.+<(?P<a>[ 0-9,-]+)>").unwrap();
    }
    input
        .lines()
        .map(|line| RE.captures(line).expect("Line was badly formatted"))
        .map(|cap| {
            Instruction {
                position: build_three_d(&cap["p"]),
                velocity: build_three_d(&cap["v"]),
                acceleration: build_three_d(&cap["a"]),
            }
        })
        .collect()
}
