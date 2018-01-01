use std::fs::File;
use std::io::Read;

#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;

struct threeD {
    x :isize,
    y: isize,
    z: isize
}

struct Instruction {
    position: threeD,
    velocity: threeD,
    acceleration: threeD
}

fn main() {
    let file_name = "../input.txt";
    let mut file = File::open(file_name).expect("Unable to open input file!");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Cannot convert file contents to string!");

    let instructions = parse_instructions(contents);
}


fn parse_instructions(input: String) -> Vec<Instruction> {
        lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<id>[a-z]+) \((?P<weight>[0-9]+)\)( -> (?P<holding>[a-z, ]+))?").unwrap();
    }
}