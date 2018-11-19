use std::fs::File;
use std::io::Read;
use std::f32;

fn main() {
    let file_name = "../input.txt";
    let mut file = File::open(file_name).expect("Unable to open input file!");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Cannot convert file contents to string!");

    let directions = contents.split(",");

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
