use crate::aoc_util;
extern crate fnv;
use fnv::FnvHashMap;
extern crate regex;
use regex::Regex;

#[derive(Debug)]
struct State {
    value: bool,
    cursor: isize,
    next_state: char,
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct StateKey {
    state_id: char,
    current_value: bool,
}

fn check(regex: &Regex, line: &str) -> Option<String> {
    if let Some(cap) = regex.captures(line.trim()) {
        return Some(cap["result"].to_string());
    }
    None
}
fn parse(lines: &str) -> (char, usize, FnvHashMap<StateKey, State>) {
    let mut start_state = ' ';
    let mut steps = 0;
    let mut states = FnvHashMap::default();

    let starting_state_check = Regex::new(r"^Begin in state (?P<result>[A-Z]).$").unwrap();
    let steps_check =
        Regex::new(r"^Perform a diagnostic checksum after (?P<result>[0-9]+) steps.$").unwrap();
    let new_state_check = Regex::new(r"^In state (?P<result>[A-Z]):$").unwrap();
    let current_value_check = Regex::new(r"^If the current value is (?P<result>[0-1]):$").unwrap();
    let write_value_check = Regex::new(r"^- Write the value (?P<result>[0-1]).$").unwrap();
    let move_value_check =
        Regex::new(r"^- Move one slot to the (?P<result>(left)|(right)).$").unwrap();
    let next_state_check = Regex::new(r"^- Continue with state (?P<result>[A-Z]).$").unwrap();

    let mut current_state = 'A';
    let mut current_value = false;
    let mut value = false;
    let mut cursor = 1;

    for line in lines.lines() {
        if let Some(state) = check(&starting_state_check, line) {
            start_state = state.chars().next().expect("This should be a char");
        }
        if let Some(found_steps) = check(&steps_check, line) {
            steps = found_steps.parse::<usize>().expect("This should be number");
        }
        if let Some(new_state) = check(&new_state_check, line) {
            current_state = new_state.chars().next().expect("Couldn't read new state");
        }
        if let Some(new_current_value) = check(&current_value_check, line) {
            current_value = new_current_value == "1";
        }
        if let Some(new_write_value) = check(&write_value_check, line) {
            value = new_write_value == "1";
        }
        if let Some(new_move_value) = check(&move_value_check, line) {
            cursor = if new_move_value == "left" { -1 } else { 1 }
        }
        if let Some(new_next_state) = check(&next_state_check, line) {
            let key = StateKey {
                state_id: current_state,
                current_value,
            };
            let state = State {
                value,
                cursor,
                next_state: new_next_state
                    .chars()
                    .next()
                    .expect("Couldn't read next state"),
            };
            states.insert(key, state);
        }
    }
    (start_state, steps, states)
}

pub fn run() {
    let lines = aoc_util::get_input(25);
    let (start_state, steps, states) = parse(&lines);
    let mut state_id = start_state;
    let mut cursor = 0;
    let mut tape = FnvHashMap::default();;

    for _ in 0..steps {
        let current_value = *tape.get(&cursor).unwrap_or(&false);
        let key = StateKey {
            current_value,
            state_id,
        };
        let state_object = states.get(&key);
        match state_object {
            Some(state_change) => {
                tape.insert(cursor, state_change.value);
                cursor += state_change.cursor;
                state_id = state_change.next_state;
            }
            None => panic!("We weren't supposed to get here"),
        }
    }
    let mut sum = 0;
    for &element in tape.values() {
        if element {
            sum += 1
        }
    }
    println!("Part 1 is {}", sum);
}
