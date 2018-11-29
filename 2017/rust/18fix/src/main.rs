extern crate aoc_util;
use std::collections::HashMap;

mod part2;

fn main() {
    let mut registers: HashMap<&str, i64> = HashMap::new();
    let lines = aoc_util::iterate_input_lines(18).collect::<Vec<&str>>();
    let mut position = 0;
    let mut played = 0;
    loop {
        let parts = lines[position]
            .split(" ")
            .filter(|x| !x.is_empty())
            .collect::<Vec<&str>>();

        match parts[0] {
            "snd" => {
                played = get_value(&registers, parts[1]);
            }
            "set" => {
                let new_value = get_value(&registers, parts[2]);
                registers.insert(parts[1], new_value);
                ()
            }
            "add" => {
                let new_value = get_value(&registers, parts[1]) + get_value(&registers, parts[2]);
                registers.insert(parts[1], new_value);
                ()
            }
            "mul" => {
                let new_value = get_value(&registers, parts[1]) * get_value(&registers, parts[2]);
                registers.insert(parts[1], new_value);
                ()
            }
            "mod" => {
                let new_value = get_value(&registers, parts[1]) % get_value(&registers, parts[2]);
                registers.insert(parts[1], new_value);
                ()
            }
            "rcv" => {
                let should_recover = get_value(&registers, parts[1]) != 0;
                if should_recover {
                    println!("1 is {:?}", registers);

                    println!("Part 1 is {}", played);
                    break;
                }
            }
            "jgz" => {
                let should_jump = get_value(&registers, parts[1]) > 0;
                if should_jump {
                    position = (position as i64 + get_value(&registers, parts[2]) - 1) as usize;
                }
            }
            _ => panic!("I don't understand {}", parts[0]),
        };
        position += 1;
        if position >= lines.len() {
            break;
        }
    }
    println!(
        "Part 2 is {}",
        part2::run(aoc_util::iterate_input_lines(18).collect::<Vec<&str>>())
    );
}

fn get_value(registers: &HashMap<&str, i64>, address_or_number: &str) -> i64 {
    address_or_number
        .parse()
        .unwrap_or(*registers.get(&address_or_number).unwrap_or(&0))
}
