use crate::aoc_util;
use crate::aoc_util::vm::*;
use fnv::FnvHashMap;

pub fn run() {
    let mut registers = FnvHashMap::default();
    let lines = aoc_util::iterate_input_lines(23);
    let instructions = lines
        .map(|line| InstructionLine::new(&line))
        .collect::<Vec<InstructionLine>>();
    let mut position = 0 as i64;
    let mut mul_used = 0;
    while position >= 0 && position < instructions.len() as i64 {
        let instruction = &instructions[position as usize];
        let register_value = get_register_value(&registers, instruction.register);
        let value = get_value(&registers, instruction.value);
        match instruction.command {
            Command::Set => registers.insert(instruction.register, value),
            Command::Sub => registers.insert(instruction.register, register_value - value),
            Command::Mul => {
                mul_used += 1;
                registers.insert(instruction.register, register_value * value)
            }
            Command::Jnz => {
                if register_value != 0 {
                    position += value - 1;
                }
                None
            }
            _ => panic!("Unknown instruction - {:?}", instruction.command),
        };
        position += 1;
    }
    println!("Part 1 is {}", mul_used);
    println!("Part 2 - see breakdown in js repo");
}
