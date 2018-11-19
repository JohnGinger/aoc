use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn main() {
    let file_name = "../input.txt";
    let mut file = File::open(file_name).expect("Unable to open input file!");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Cannot convert file contents to string!");


    let mut registers: HashMap<&str, isize> = HashMap::new();
    let lines = contents.lines().collect::<Vec<&str>>();
    let mut position = 0;
    let mut mul_used = 0;
    loop {
        let parts = lines[position]
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