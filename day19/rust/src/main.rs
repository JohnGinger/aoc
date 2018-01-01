use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

#[derive(PartialEq)]
enum Direction {
    Vertical,
    Horizontal,
    Both,
}

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

struct Point {
    direction: Direction,
    letter: Option<char>,
}

fn main() {
    let file_name = "../input.txt";
    let mut file = File::open(file_name).expect("Unable to open input file!");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Cannot convert file contents to string!");

    let points = parse_input(contents);
    let mut position = Position { x: 0, y: 0 };
    for (position_to_check, point) in &points {
        if position_to_check.y == 0 && point.direction == Direction::Vertical {
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
                match point.letter {
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

        match going {
            Going::Up => if points.contains_key(&up) {
                position = up
            } else if points.contains_key(&left) {
                going = Going::Left;
                position = left
            } else if points.contains_key(&right) {
                going = Going::Right;
                position = right
            } else {
                break;
            },
            Going::Down => if points.contains_key(&down) {
                position = down
            } else if points.contains_key(&left) {
                going = Going::Left;
                position = left
            } else if points.contains_key(&right) {
                going = Going::Right;
                position = right
            } else {
                break;
            },
            Going::Left => if points.contains_key(&left) {
                position = left
            } else if points.contains_key(&up) {
                going = Going::Up;
                position = up
            } else if points.contains_key(&down) {
                going = Going::Down;
                position = down
            } else {
                break;
            },
            Going::Right => if points.contains_key(&right) {
                position = right
            } else if points.contains_key(&up) {
                going = Going::Up;
                position = up
            } else if points.contains_key(&down) {
                going = Going::Down;
                position = down
            } else {
                break;
            },
        }
    }
    println!("Part 1 is {}", visited.into_iter().collect::<String>());
    println!("Part 2 is {}", steps)
}

fn parse_input(input: String) -> HashMap<Position, Point> {
    let mut points = HashMap::new();
    let mut y = 0;
    for line in input.lines() {
        let mut x = 0;
        for point in line.chars() {
            let position = Position { x, y };
            match point {
                '|' => {
                    points.insert(
                        position,
                        Point {
                            direction: Direction::Vertical,
                            letter: None,
                        },
                    );
                    ()
                }
                '-' => {
                    points.insert(
                        position,
                        Point {
                            direction: Direction::Horizontal,
                            letter: None,
                        },
                    );
                    ()
                }
                '+' => {
                    points.insert(
                        position,
                        Point {
                            direction: Direction::Both,
                            letter: None,
                        },
                    );
                    ()
                }
                ' ' => (), // Do nothing,
                letter => {
                    points.insert(
                        position,
                        Point {
                            direction: Direction::Both,
                            letter: Some(letter),
                        },
                    );
                    ()
                }
            }
            x += 1;
        }
        y += 1;
    }
    points
}
