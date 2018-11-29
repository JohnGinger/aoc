extern crate aoc_util;
use std::collections::HashMap;

fn main() {
    let mut registers: HashMap<&str, isize> = HashMap::new();
    let lines = aoc_util::get_input_lines(23);
    let mut position = 0;
    let mut mul_used = 0;
    loop {
        let l = lines.clone();
        let parts = l[position]
            .split(" ")
            .filter(|x| !x.is_empty())
            .collect::<Vec<&str>>();

        match parts[0] {
            "set" => {
                let new_value = get_value(&registers, parts[2]);
                registers.insert(parts[1], new_value);
                ()
            }
            "sub" => {
                let new_value = get_value(&registers, parts[1]) - get_value(&registers, parts[2]);
                registers.insert(parts[1], new_value);
                ()
            }
            "mul" => {
                mul_used += 1;
                let new_value = get_value(&registers, parts[1]) * get_value(&registers, parts[2]);
                registers.insert(parts[1], new_value);
                ()
            }
            "jnz" => {
                let should_jump = get_value(&registers, parts[1]) != 0;
                if should_jump {
                    position = (position as isize + get_value(&registers, parts[2]) - 1) as usize;
                }
            }
            _ => panic!("I don't understand {}", parts[0]),
        };
        position += 1;
        if position >= lines.len() {
            break;
        }
    }
    println!("Part 1 is {}", mul_used);
}

fn get_value(registers: &HashMap<&str, isize>, address_or_number: &str) -> isize {
    address_or_number
        .parse()
        .unwrap_or(*registers.get(&address_or_number).unwrap_or(&0))
}
