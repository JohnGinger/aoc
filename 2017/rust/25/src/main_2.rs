extern crate aoc_util;
use std::collections::HashMap;
#[macro_use]
extern crate nom;
use nom::anychar;
use nom::digit;
use nom::is_alphabetic;

#[derive(Debug)]
struct State {
    value: bool,
    cursor: isize,
    next_state: char,
}

struct MetaState {
    state: State,
    current_value: bool,
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct StateKey {
    state_id: char,
    current_value: bool,
}

named!(pub num(&str) -> i64, do_parse!(
	s: recognize!(
		pair!(
			opt!(tag_s!("-")),
			call!(digit)
		)
	) >> (s.parse().unwrap())
));

named!(pub direction(&str) -> isize, do_parse!(
	s: alt!(
        tag_s!("left") => { |_| -1 } |
        tag_s!("right") => { |_| 1 }
    ) >> (s)
));

named!(pub as_bool(&str) -> bool, do_parse!(
	s: alt!(
        tag_s!("0") => { |_| false } |
        tag_s!("1") => { |_| true }
    ) >> (s)
));

named!(state_block(&str) -> State, ws!(do_parse!(
    tag_s!("- Write the value ")
    >> new_value: terminated!(as_bool, tag_s!("."))
    >> tag_s!("- Move one slot to the ")
    >> cursor: terminated!(direction, tag_s!("."))
    >> tag_s!("- Continue with state")
    >> next_state: terminated!(anychar, tag_s!("."))
    >> (State {
            value: new_value,
            cursor: cursor,
            next_state: next_state
        })         
)));

fn parse(lines: String) -> (char, usize, HashMap<StateKey, State>) {
    let (states, (start_state, steps)) = ws!(
        lines.as_str(),
        do_parse!(
            tag_s!("Begin in state")
                >> start: terminated!(anychar, tag_s!("."))
                >> tag_s!("Perform a diagnostic checksum after")
                >> steps: terminated!(num, tag_s!("steps."))
                >> ((start, steps as usize))
        )
    )
    .expect("Couldn't parse start and steps");

    let state_blocks = states.split("\n\n").map(|x| x.to_string());

    let mut program_states = HashMap::new();
    for state in state_blocks {
        for (index,line) in state.lines().enumerate(){
            let mut state_id = ' ';
            let mut off_value = false;
            let mut off_cursor =0;
            let mut off_state = ' ';
            match index {
                0 => state_id = *line.get(10 as usize).unwrap(),
                2 => off_value = *line.get(23 as usize).unwrap() == 1,
                3 =>  off_cursor= *line.get(28 as usize).unwrap() == 'r',
                4 => off_state = *line.get(28 as usize).unwrap()

            }
        }
    }

    //println!("states {} {}", start, steps);
    /*
    for (state_id, state_info_vec) in states {
        println!("states {}", state_id);
        for x in state_info_vec {
            let key = StateKey {
                state_id: state_id,
                current_value: x.current_value,
            };
            program_states.insert(key, x.state);
        }
    }
    */

    (start_state, 0, HashMap::new())
}

fn main() {
    let lines = aoc_util::get_input(25);
    let (start_state, steps, states) = parse(lines);
    let mut state_id = start_state;
    let mut cursor = 0;
    let mut tape = HashMap::new();
    println!("{:?}", states);

    for _ in 0..steps {
        let current_value = *tape.get(&cursor).unwrap_or(&false);
        let key = StateKey {
            current_value: current_value,
            state_id: state_id,
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
    for (_, element) in &tape {
        if *element {
            sum += 1
        }
    }
    println!("Part 1 is {}", sum);
}
