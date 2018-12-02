use crate::aoc_util;
use std::f32;

pub fn run() {
    let contents = aoc_util::get_input(11);
    let directions = contents.split(',');

    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    let mut max_dist = 0.0;

    for direction in directions {
        match direction {
            "ne" => {
                x += 0.5;
                y += 0.5
            }
            "n" => y += 1.0,
            "nw" => {
                x -= 0.5;
                y += 0.5
            }
            "sw" => {
                x -= 0.5;
                y -= 0.5
            }
            "s" => y -= 1.0,
            "se" => {
                x += 0.5;
                y -= 0.5
            }
            _ => panic!("That isn't a direction"),
        }
        if (x.abs() + y.abs()) > max_dist {
            max_dist = x.abs() + y.abs();
        }
    }
    println!("Part 1 is {}", x.abs() + y.abs());
    println!("Part 2 is {}", max_dist);
}
