extern crate aoc_util;
use std::collections::HashMap;
#[macro_use]
extern crate nom;
use nom::anychar;
use nom::digit;
use nom::is_alphabetic;
use nom::types::CompleteStr;

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

use nom::{alpha, alphanumeric, space, IResult};

named!(
    name_parser<&str>,
    do_parse!(tag!("hello ") >> name: map_res!(alphanumeric, std::str::from_utf8) >> (name))
);

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

named!(state_block(&str) -> (bool, State), ws!(do_parse!(
	tag_s!("If the current value is ")
    >> current_value: terminated!(as_bool, tag_s!(":"))
    >> tag_s!("- Write the value ")
    >> new_value: terminated!(as_bool, tag_s!("."))
    >> tag_s!("- Move one slot to the ")
    >> cursor: terminated!(direction, tag_s!("."))
    >> tag_s!("- Continue with state")
    >> next_state: terminated!(anychar, tag_s!("."))
    >> (current_value,
        State {
            value: new_value,
            cursor: cursor,
            next_state: next_state
        })          
)));

named!(state_block2(&str) -> State, ws!(do_parse!(
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
    let (states, (start_state, steps, s2)) = ws!(
        lines.as_str(),
        do_parse!(
            tag_s!("Begin in state")
                >> start: terminated!(anychar, tag_s!("."))
                >> tag_s!("Perform a diagnostic checksum after")
                >> steps: terminated!(num, tag_s!("steps."))
                >> states:
                many0!(ws!(do_parse!(
                    tag_s!("In state")
                        >> state: terminated!(anychar, tag_s!(":"))
                        >> transitions:
                            many1!(ws!(do_parse!(
                                tag_s!("If the current value is")
                                    >> current_value: terminated!(as_bool, tag_s!(":"))
                                    >> tag_s!("- Write the value")
                                    >> next_value: terminated!(as_bool, tag_s!("."))
                                    >> tag_s!("- Move one slot to the")
                                    >> cursor:
                                        terminated!(
                                            alt!(
                                            tag_s!("left") => { |_| -1 } |
                                            tag_s!("right") => { |_| 1 }
                                        ),
                                            tag_s!(".")
                                        )
                                    >> tag_s!("- Continue with state")
                                    >> next_state: terminated!(anychar, tag_s!("."))
                                    >> (next_state)
                            )))
                        >> (state, transitions)
                )))
                >> ((start, steps as usize, states))
        )
    )
    .expect("Couldn't parse start and steps");
/*
    let state_blocks = states.split("\n\n");

    for state in state_blocks {
        let res = ws!(
            state,
            do_parse!(
                tag_s!("In state ")
                    >> start: terminated!(anychar, tag_s!(":"))
                    >> blah: many1!(state_block)
                    >> (
                        start,
                        blah
                    )
            )
        )
        .expect("Couldn't parse state");
        println!(" {:?}", res);
        println!("")
    }
*/


    println!("states {} {} {:?}", start_state, steps, states);
    /*
    let mut program_states = HashMap::new();
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
