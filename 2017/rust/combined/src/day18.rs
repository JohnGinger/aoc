use crate::aoc_util::iterate_input_lines;
use crate::aoc_util::vm::*;

use fnv::FnvHashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

fn part1(instructions: &[InstructionLine]) {
    let mut registers = FnvHashMap::default();
    let mut position = 0 as i64;
    let mut played = 0;
    while position >= 0 && position < instructions.len() as i64 {
        let instruction = &instructions[position as usize];
        let register_value = get_register_value(&registers, instruction.register);
        let value = get_value(&registers, instruction.value);
        match instruction.command {
            Command::Set => registers.insert(instruction.register, value),
            Command::Add => registers.insert(instruction.register, register_value + value),
            Command::Mul => registers.insert(instruction.register, register_value * value),
            Command::Mod => registers.insert(instruction.register, register_value % value),
            Command::Snd => {
                played = register_value;
                None
            }
            Command::Rcv => {
                if register_value != 0 {
                    println!("Part 1 is {}", played);
                    break;
                }
                None
            }
            Command::Jgz => {
                if register_value > 0 {
                    position += value - 1;
                }
                None
            }
            _ => panic!("Unknown instruction - {:?}", instruction.command),
        };
        position += 1;
    }
}

struct DuetPlayingProgram {
    registers: FnvHashMap<char, i64>,
    instructions: Vec<InstructionLine>,
    sender: Sender<i64>,
    receiver: Receiver<i64>,
    position: i64,
}

impl DuetPlayingProgram {
    fn new(
        instructions: &[InstructionLine],
        program_id: i64,
        sender: Sender<i64>,
        receiver: Receiver<i64>,
    ) -> Self {
        let mut registers = FnvHashMap::default();
        registers.insert('p', program_id);
        DuetPlayingProgram {
            registers,
            instructions: instructions.to_vec(),
            sender,
            receiver,
            position: 0,
        }
    }
    fn next(&mut self) -> i64 {
        let mut sent = 0;
        while self.position >= 0 && self.position < self.instructions.len() as i64 {
            let instruction = &self.instructions[self.position as usize];
            let register = instruction.register;
            let register_value = get_register_value(&self.registers, register);

            let value = get_value(&self.registers, instruction.value);
            match instruction.command {
                Command::Set => self.registers.insert(register, value),
                Command::Add => self.registers.insert(register, register_value + value),
                Command::Mul => self.registers.insert(register, register_value * value),
                Command::Mod => self.registers.insert(register, register_value % value),
                Command::Snd => {
                    self.sender
                        .send(register_value)
                        .expect("There wasn't something to send to");
                    sent += 1;
                    None
                }
                Command::Rcv => match self.receiver.recv_timeout(Duration::from_millis(100)) {
                    Ok(val) => self.registers.insert(instruction.register, val),
                    Err(_) => return sent,
                },
                Command::Jgz => {
                    if register_value > 0 {
                        self.position += value - 1;
                    }
                    None
                }
                _ => panic!("Unknown instruction - {:?}", instruction.command),
            };
            self.position += 1;
        }
        sent
    }
}

fn part2(instructions: &[InstructionLine]) {
    let (tx1, rx1) = channel();
    let (tx2, rx2) = channel();
    let mut p0 = DuetPlayingProgram::new(instructions, 0, tx1, rx2);
    let mut p1 = DuetPlayingProgram::new(instructions, 1, tx2, rx1);

    thread::spawn(move || p0.next());
    let child2 = thread::spawn(move || p1.next());

    println!("Part 2 is {}", child2.join().unwrap());
}

pub fn run() {
    let lines = iterate_input_lines(18);
    let instructions: Vec<InstructionLine> =
        lines.map(|line| InstructionLine::new(&line)).collect();
    part1(&instructions);
    part2(&instructions);
}
