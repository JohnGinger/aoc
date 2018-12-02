use fnv::FnvHashMap;

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
    pub fn new(line: &str) -> InstructionLine {
        let parts = line
            .split(' ')
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

pub fn get_register_value(registers: &FnvHashMap<char, i64>, register: char) -> i64 {
    match register.to_digit(10) {
        Some(num) => i64::from(num),
        None => *registers.get(&register).unwrap_or(&0),
    }
}

pub fn get_value(registers: &FnvHashMap<char, i64>, value: (Option<i64>, char)) -> i64 {
    match value {
        (Some(num), _) => num,
        (None, register) => *registers.get(&register).unwrap_or(&0),
    }
}
