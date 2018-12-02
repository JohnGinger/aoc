use std::collections::HashMap;
use std::fs::File;
use std::io::*;

pub fn get_input(day: i32) -> String {
    let file_name = format!("../../data/{}.txt", day);
    let mut file = File::open(file_name).expect("Unable to open input file!");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Cannot convert file contents to string!");

    String::from(contents.trim())
}

pub fn get_input_lines(day: i32) -> Vec<String> {
    let file_name = format!("../../data/{}.txt", day);
    let file = File::open(file_name).expect("Unable to open input file!");

    BufReader::new(file).lines().map(|l| l.unwrap()).collect()
}

pub struct PuzzleLines {
    lines: Lines<BufReader<File>>,
}

impl PuzzleLines {
    fn new(file_name: String) -> Self {
        let file = File::open(file_name).expect("Unable to open input file!");
        PuzzleLines {
            lines: BufReader::new(file).lines(),
        }
    }
}

impl Iterator for PuzzleLines {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        match self.lines.next() {
            Some(result) => Some(result.expect("could not read input line")),
            None => None,
        }
    }
}

pub fn iterate_input_lines(day: i32) -> PuzzleLines {
    let file_name = format!("../../data/{}.txt", day);
    PuzzleLines::new(file_name)
}

#[derive(Debug, Clone)]
pub enum Command {
    Snd,
    Set,
    Add,
    Mul,
    Mod,
    Rcv,
    Jgz,
    Jnz,
    Sub,
}

#[derive(Debug, Clone)]
pub struct InstructionLine {
    pub command: Command,
    pub register: char,
    pub value: (Option<i64>, char),
}

impl InstructionLine {
    pub fn new(line: String) -> InstructionLine {
        let parts = line
            .split(" ")
            .filter(|x| !x.is_empty())
            .collect::<Vec<&str>>();
        let register = parts[1].chars().next().unwrap();
        let mut value = (None, ' ');
        if parts.len() == 3 {
            value = match parts[2].parse::<i64>() {
                Ok(val) => (Some(val), parts[2].chars().next().unwrap()),
                Err(_) => (None, parts[2].chars().next().unwrap()),
            }
        }
        InstructionLine {
            command: get_instruction(parts[0]),
            register: register,
            value: value,
        }
    }
}

pub fn get_instruction(instruction: &str) -> Command {
    match instruction {
        "snd" => Command::Snd,
        "set" => Command::Set,
        "add" => Command::Add,
        "mul" => Command::Mul,
        "mod" => Command::Mod,
        "rcv" => Command::Rcv,
        "jgz" => Command::Jgz,
        "jnz" => Command::Jnz,
        "sub" => Command::Sub,
        unknown => panic!("Strange instruction - {}", unknown),
    }
}

pub fn get_register_value(registers: &HashMap<char, i64>, register: char) -> i64 {
    match register.to_digit(10) {
        Some(num) => num as i64,
        None => *registers.get(&register).unwrap_or(&0),
    }
}

pub fn get_value(registers: &HashMap<char, i64>, value: (Option<i64>, char)) -> i64 {
    match value {
        (Some(num), _) => num,
        (None, register) => *registers.get(&register).unwrap_or(&0),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
