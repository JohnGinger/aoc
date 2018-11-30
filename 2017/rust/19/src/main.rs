extern crate aoc_util;
use std::collections::HashMap;

#[derive(PartialEq)]
enum Going {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: usize, y: usize) -> Position {
        Position {
            x: x as isize,
            y: y as isize,
        }
    }
}

fn main() {
    let contents = aoc_util::get_input_lines(19);

    let points = parse_input(contents);
    let mut position = Position { x: 0, y: 0 };
    for (position_to_check, _) in &points {
        if position_to_check.y == 0 {
            position = Position {
                x: position_to_check.x,
                y: position_to_check.y,
            };
            break;
        }
    }

    let mut going = Going::Down;
    let mut visited = vec![];
    let mut steps = 0;
    loop {
        steps += 1;
        match points.get(&position) {
            Some(point) => {
                match point {
                    Some(letter) => visited.push(letter),
                    None => (), // Nothing here
                }
            }
            None => panic!("We have managed to visit a square with nothing on it"),
        }

        let left = Position {
            x: position.x - 1,
            y: position.y,
        };
        let has_left = points.contains_key(&left);

        let right = Position {
            x: position.x + 1,
            y: position.y,
        };
        let has_right = points.contains_key(&right);

        let up = Position {
            x: position.x,
            y: position.y - 1,
        };
        let has_up = points.contains_key(&up);

        let down = Position {
            x: position.x,
            y: position.y + 1,
        };
        let has_down = points.contains_key(&down);

        match going {
            Going::Up => {
                if has_up {
                    position = up
                } else if has_left {
                    going = Going::Left;
                    position = left
                } else if has_right {
                    going = Going::Right;
                    position = right
                } else {
                    break;
                }
            }
            Going::Down => {
                if has_down {
                    position = down
                } else if has_left {
                    going = Going::Left;
                    position = left
                } else if has_right {
                    going = Going::Right;
                    position = right
                } else {
                    break;
                }
            }
            Going::Left => {
                if has_left {
                    position = left
                } else if has_up {
                    going = Going::Up;
                    position = up
                } else if has_down {
                    going = Going::Down;
                    position = down
                } else {
                    break;
                }
            }
            Going::Right => {
                if has_right {
                    position = right
                } else if has_up {
                    going = Going::Up;
                    position = up
                } else if has_down {
                    going = Going::Down;
                    position = down
                } else {
                    break;
                }
            }
        }
    }
    println!("Part 1 is {}", visited.into_iter().collect::<String>());
    println!("Part 2 is {}", steps)
}

fn parse_input(lines: Vec<String>) -> HashMap<Position, Option<char>> {
    let mut points = HashMap::new();
    for (y, line) in lines.into_iter().enumerate() {
        for (x, point) in line.chars().enumerate() {
            match point {
                '|' | '-' | '+' => points.insert(Position::new(x, y), None),
                ' ' => None, // Do nothing,
                letter => points.insert(Position::new(x, y), Some(letter)),
            };
        }
    }
    points
}
