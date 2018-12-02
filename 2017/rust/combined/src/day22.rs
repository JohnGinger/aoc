use crate::aoc_util;
use std::collections::HashMap;

#[derive(PartialEq)]
enum Going {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug, Eq, PartialEq, Hash)]
enum VirusState {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn from_usize(x: usize, y: usize) -> Position {
        Position {
            x: x as isize,
            y: y as isize,
        }
    }
}

pub fn run() {
    let mut going = Going::Up;
    let mut position = Position { x: 12, y: 12 };

    let mut map = HashMap::new();
    let mut map2 = HashMap::new();

    for (y, line) in aoc_util::iterate_input_lines(22).enumerate() {
        for (x, letter) in line.chars().enumerate() {
            match letter {
                '#' => {
                    map.insert(Position::from_usize(x, y), true);
                    map2.insert(Position::from_usize(x, y), VirusState::Infected);
                }
                '.' => {
                    map.insert(Position::from_usize(x, y), false);
                    map2.insert(Position::from_usize(x, y), VirusState::Clean);
                }
                _ => panic!("The map isn't valid"),
            }
        }
    }
    let mut infected_count_one = 0;
    for _ in 0..10000 {
        let left = Position {
            x: position.x - 1,
            y: position.y,
        };

        let right = Position {
            x: position.x + 1,
            y: position.y,
        };

        let up = Position {
            x: position.x,
            y: position.y - 1,
        };

        let down = Position {
            x: position.x,
            y: position.y + 1,
        };

        if *map.get(&position).unwrap_or(&false) {
            map.insert(position, false);
            match going {
                Going::Up => {
                    going = Going::Right;
                    position = right
                }
                Going::Down => {
                    going = Going::Left;
                    position = left
                }
                Going::Left => {
                    going = Going::Up;
                    position = up
                }
                Going::Right => {
                    going = Going::Down;
                    position = down
                }
            }
        } else {
            map.insert(position, true);
            infected_count_one += 1;
            match going {
                Going::Up => {
                    going = Going::Left;
                    position = left
                }
                Going::Down => {
                    going = Going::Right;
                    position = right
                }
                Going::Left => {
                    going = Going::Down;
                    position = down
                }
                Going::Right => {
                    going = Going::Up;
                    position = up
                }
            }
        }
    }
    println!("Part 1 is {}", infected_count_one);

    let mut infected_count_two = 0;
    position = Position { x: 12, y: 12 };
    going = Going::Up;
    for _ in 0..10_000_000 {
        /*
        Debug
        for j in -3..6 {
            for i in -3..6 {
                if position.x == i && position.y == j {
                    print!("[")
                }
                match *map2.get(&Position {x:i, y:j}).unwrap_or(&VirusState::Clean){
                    VirusState::Clean => print!("."),
                    VirusState::Flagged => print!("F"),
                    VirusState::Weakened => print!("W"),
                    VirusState::Infected => print!("#")
                }
                if position.x == i && position.y == j {
                    print!("]")
                }
            }
            println!("")
        }
            println!("");
            println!("");
            println!("");

            */

        let left = Position {
            x: position.x - 1,
            y: position.y,
        };

        let right = Position {
            x: position.x + 1,
            y: position.y,
        };

        let up = Position {
            x: position.x,
            y: position.y - 1,
        };

        let down = Position {
            x: position.x,
            y: position.y + 1,
        };

        match *map2.get(&position).unwrap_or(&VirusState::Clean) {
            VirusState::Clean => {
                map2.insert(position, VirusState::Weakened);
                match going {
                    Going::Up => {
                        going = Going::Left;
                        position = left
                    }
                    Going::Down => {
                        going = Going::Right;
                        position = right
                    }
                    Going::Left => {
                        going = Going::Down;
                        position = down
                    }
                    Going::Right => {
                        going = Going::Up;
                        position = up
                    }
                }
            }
            VirusState::Weakened => {
                map2.insert(position, VirusState::Infected);
                infected_count_two += 1;
                match going {
                    Going::Up => position = up,
                    Going::Down => position = down,
                    Going::Left => position = left,
                    Going::Right => position = right,
                }
            }
            VirusState::Infected => {
                map2.insert(position, VirusState::Flagged);
                match going {
                    Going::Up => {
                        going = Going::Right;
                        position = right
                    }
                    Going::Down => {
                        going = Going::Left;
                        position = left
                    }
                    Going::Left => {
                        going = Going::Up;
                        position = up
                    }
                    Going::Right => {
                        going = Going::Down;
                        position = down
                    }
                }
            }
            VirusState::Flagged => {
                map2.insert(position, VirusState::Clean);
                match going {
                    Going::Up => {
                        going = Going::Down;
                        position = down
                    }
                    Going::Down => {
                        going = Going::Up;
                        position = up
                    }
                    Going::Left => {
                        going = Going::Right;
                        position = right
                    }
                    Going::Right => {
                        going = Going::Left;
                        position = left
                    }
                }
            }
        }
    }
    println!("Part 2 is {}", infected_count_two);
}
