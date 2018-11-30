extern crate aoc_util;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

// mod part2;
#[derive(Debug, Clone)]
enum Instructions {
    Snd,
    Set,
    Add,
    Mul,
    Mod,
    Rcv,
    Jgz,
}

#[derive(Debug, Clone)]
struct InstructionLine {
    command: Instructions,
    register: char,
    value: (Option<i64>, char),
}

fn get_instruction(instruction: &str) -> Instructions {
    match instruction {
        "snd" => Instructions::Snd,
        "set" => Instructions::Set,
        "add" => Instructions::Add,
        "mul" => Instructions::Mul,
        "mod" => Instructions::Mod,
        "rcv" => Instructions::Rcv,
        "jgz" => Instructions::Jgz,
        _ => panic!("Strange instruction"),
    }
}

impl InstructionLine {
    fn new(line: String) -> InstructionLine {
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

fn get_register_value(registers: &HashMap<char, i64>, register: char) -> i64 {
    match register.to_digit(10) {
        Some(num) => num as i64,
        None => *registers.get(&register).unwrap_or(&0),
    }
}

fn get_value(registers: &HashMap<char, i64>, value: (Option<i64>, char)) -> i64 {
    match value {
        (Some(num), _) => num,
        (None, register) => *registers.get(&register).unwrap_or(&0),
    }
}

fn part1(instructions: &Vec<InstructionLine>) {
    let mut registers: HashMap<char, i64> = HashMap::new();
    let mut position = 0 as i64;
    let mut played = 0;
    while position >= 0 && position < instructions.len() as i64 {
        let instruction = &instructions[position as usize];
        let register_value = get_register_value(&registers, instruction.register);
        let value = get_value(&registers, instruction.value);
        match instruction.command {
            Instructions::Set => registers.insert(instruction.register, value),
            Instructions::Add => registers.insert(instruction.register, register_value + value),
            Instructions::Mul => registers.insert(instruction.register, register_value * value),
            Instructions::Mod => registers.insert(instruction.register, register_value % value),
            Instructions::Snd => {
                played = register_value;
                None
            }
            Instructions::Rcv => {
                if register_value != 0 {
                    println!("Part 1 is {}", played);
                    break;
                }
                None
            }
            Instructions::Jgz => {
                if register_value > 0 {
                    position = position + value - 1;
                }
                None
            }
        };
        position += 1;
    }
}

struct DuetPlayingProgram {
    registers: HashMap<char, i64>,
    instructions: Vec<InstructionLine>,
    sender: Sender<i64>,
    receiver: Receiver<i64>,
    position: i64,
}

impl DuetPlayingProgram {
    fn new(
        instructions: &Vec<InstructionLine>,
        program_id: i64,
        sender: Sender<i64>,
        receiver: Receiver<i64>,
    ) -> Self {
        let mut registers = HashMap::new();
        registers.insert('p', program_id);
        DuetPlayingProgram {
            registers: registers,
            instructions: instructions.to_vec(),
            sender: sender,
            receiver: receiver,
            position: 0,
        }
    }
    fn next(&mut self) -> i64 {
        let mut sent = 0;
        while self.position >= 0 && self.position < self.instructions.len() as i64 {
            let instruction = &self.instructions[self.position as usize];
            let register_value = get_register_value(&self.registers, instruction.register);
            let value = get_value(&self.registers, instruction.value);
            match instruction.command {
                Instructions::Set => self.registers.insert(instruction.register, value),
                Instructions::Add => self
                    .registers
                    .insert(instruction.register, register_value + value),
                Instructions::Mul => self
                    .registers
                    .insert(instruction.register, register_value * value),
                Instructions::Mod => self
                    .registers
                    .insert(instruction.register, register_value % value),
                Instructions::Snd => {
                    self.sender
                        .send(register_value)
                        .expect("There wasn't something to send to");
                    sent += 1;
                    None
                }
                Instructions::Rcv => match self.receiver.recv_timeout(Duration::from_millis(100)) {
                    Ok(val) => self.registers.insert(instruction.register, val),
                    Err(_) => return sent,
                },
                Instructions::Jgz => {
                    if register_value > 0 {
                        self.position = self.position + value - 1;
                    }
                    None
                }
            };
            self.position += 1;
        }
        return sent;
    }
}

fn part2(instructions: &Vec<InstructionLine>) {
    let (tx1, rx1) = channel();
    let (tx2, rx2) = channel();
    let mut p0 = DuetPlayingProgram::new(instructions, 0, tx1, rx2);
    let mut p1 = DuetPlayingProgram::new(instructions, 1, tx2, rx1);

    thread::spawn(move || p0.next());
    let child2 = thread::spawn(move || p1.next());

    println!("Part 2 is {}", child2.join().unwrap());
}

fn main() {
    let lines = aoc_util::iterate_input_lines(18);
    let instructions = lines.map(|line| InstructionLine::new(line)).collect();
    part1(&instructions);
    part2(&instructions);
}
