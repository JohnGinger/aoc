use std::collections::HashMap;

#[derive(Debug)]
struct Instruction {
    id: usize,
    register: Option<char>,
    arg1: (Option<char>, Option<isize>),
    arg2: (Option<char>, Option<isize>),
}

fn get_arg_value(
    registers: &HashMap<char, isize>,
    instruction_arg: (Option<char>, Option<isize>),
) -> isize {
    match instruction_arg {
        (Some(instruction), None) => *registers.get(&instruction).unwrap_or(&0),
        (None, Some(value)) => value,
        _ => panic!("Instruction with invalid number and address"),
    }
}

fn parse_instructions(lines: Vec<&str>) -> Vec<Instruction> {
    let instructions = vec!["snd", "set", "add", "mul", "mod", "rcv", "jgz"];
    lines
        .iter()
        .map(|line| {
            let parts = line.split(" ")
                .filter(|x| !x.is_empty())
                .collect::<Vec<&str>>();
            let id = instructions.iter().position(|&r| r == parts[0]).unwrap();
            let part1 = parts[1].chars().next();

            if parts.len() == 2 {
                Instruction {
                    id,
                    register: part1,
                    arg1: (part1, None),
                    arg2: (None, None),
                }
            } else {
                let part2 = parts[2].chars().next();
                match (parts[1].parse(), parts[2].parse()) {
                    (Ok(one), Ok(two)) => Instruction {
                        id,
                        register: None,
                        arg1: (None, Some(one)),
                        arg2: (None, Some(two)),
                    },
                    (Err(_), Ok(two)) => Instruction {
                        id,
                        register: part1,
                        arg1: (part1, None),
                        arg2: (None, Some(two)),
                    },
                    (Ok(one), Err(_)) => Instruction {
                        id,
                        register: None,
                        arg1: (None, Some(one)),
                        arg2: (part2, None),
                    },
                    (Err(_), Err(_)) => Instruction {
                        id,
                        register: part1,
                        arg1: (part1, None),
                        arg2: (part2, None),
                    },
                }
            }
        })
        .collect()
}

pub fn run(instructions: Vec<&str>) -> usize {
    let instructions = parse_instructions(instructions);
    let mut registers0: HashMap<char, isize> = HashMap::new();
    let mut registers1: HashMap<char, isize> = HashMap::new();
    registers1.insert('p', 1);
/*
    let mut from1 = vec![];
    let mut from0 = vec![];
    let mut program_1_sends = 0;

    let mut position1 =
        add_next_value_to_queue(&mut registers1, &instructions, &mut from0, &mut from1, 0);
    println!("1 is {:?}, {:?}", registers1, from1);

    if from1.len() > 0 {
        program_1_sends += 1;
    }

    let mut position0 =
        add_next_value_to_queue(&mut registers0, &instructions, &mut from1, &mut from0, 0);
    println!("0 is {:?},  {:?}", registers0, from0);



    loop {
        position1 = add_next_value_to_queue(
            &mut registers1,
            &instructions,
            &mut from0,
            &mut from1,
            position1,
        );
        println!("1 is {:?}", registers1);
        program_1_sends += from1.len();
        if from1.len() == 0 {
            return program_1_sends;
        }
        position0 = add_next_value_to_queue(
            &mut registers0,
            &instructions,
            &mut from1,
            &mut from0,
            position0,
        );
        println!("0 is {:?}", registers0);
        if from0.len() == 0 {
            return program_1_sends;
        }
    }*/

    let dueter0 = Dueter {registers: registers0,  position:0, instructions, received :vec![]}.iter();
    let dueter1 = Dueter {registers: registers1,  position:0, instructions, received :vec![]}.iter();

    let mut deuter0_sent = None;
    let mut dueter1_sent = None;
    let mut num_dueter1_sent = 0;
    loop {
        deuter0_sent = dueter0.next();
        dueter1_sent = dueter1.next();
        dueter0.receive(dueter1_sent);
                dueter1.receive(deuter0_sent);

        match (deuter0_sent, dueter1_sent) {
            (None, None) => break,
            (Some(_), None) => num_dueter1_sent +=1,
            (Some(_), Some(_)) => num_dueter1_sent +=1,
            _ => (), // The program can still keep going
        }
    }
    num_dueter1_sent;
}


struct Dueter {
    registers: HashMap<char, isize>,
    position: usize,
    instructions: Vec<Instruction>,
    received: Vec<isize>,
}

impl Iterator for Dueter {
    type Item = u32;

    fn receive(received: Vec<isize>){
match received {
            Some(received) => self.received.push(received),
            None => ()
        }
    }
    fn next(&mut self, ) -> Option<isize> {
        let instruction = &self.instructions[self.position];
        let mut to_send = None;
        
        loop {
            match instruction.id {
                0 => {
                    to_send = get_arg_value(&self.registers, instruction.arg1);
                    break;
                }
                1 => {
                    let new_value = get_arg_value(&self.registers, instruction.arg2);
                    self.registers.insert(
                        instruction.register.expect("Set called without a register"),
                        new_value,
                    );
                    ()
                }
                2 => {
                    let new_value = get_arg_value(&self.registers, instruction.arg1)
                        + get_arg_value(&self.registers, instruction.arg2);
                    self.registers.insert(
                        instruction.register.expect("Add called without a register"),
                        new_value,
                    );
                    ()
                }
                3 => {
                    let new_value = get_arg_value(&self.registers, instruction.arg1)
                        * get_arg_value(&self.registers, instruction.arg2);
                    self.registers.insert(
                        instruction.register.expect("Mul called without a register"),
                        new_value,
                    );
                    ()
                }
                4 => {
                    let new_value = get_arg_value(&self.registers, instruction.arg1)
                        % get_arg_value(&self.registers, instruction.arg2);
                    self.registers.insert(
                        instruction.register.expect("Mod called without a register"),
                        new_value,
                    );
                    ()
                }
                5 => {
                    println!("hit 5");
match self.received {
Some(x) => self.registers.insert(
                            instruction.register.expect("Rcv called without a register"),
                            x,
                        ),
                        None => break
}                        

                   
                }
                6 => {
                    let should_jump = get_arg_value(&self.registers, instruction.arg1) > 0;
                    if should_jump {
                        self.position = (self.position as isize + get_arg_value(&self.registers, instruction.arg2)
                            - 1) as usize;
                    }
                }
                _ => panic!("I don't understand {:?}", instruction),
            };
            self.position += 1;
            if self.position >= self.instructions.len() {
                break;
            }
        }
        Some(to_send)
    }
}
