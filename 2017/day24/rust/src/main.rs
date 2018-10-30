use std::fs::File;
use std::io::Read;

fn main() {
    let file_name = "../input.txt";
    let mut file = File::open(file_name).expect("Unable to open input file!");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Cannot convert file contents to string!");

    let parts = contents.lines().map(|line| {
        let sides = line.trim().split("/").map(|x| x.parse().expect("Should be a number")).collect::<Vec<usize>>();
        Part {
            x: sides[0],
            y: sides[1],
        }
        }).collect::<Vec<Part>>();


    println!("Part 1 is {:?}", get_value(Part{x: 0, y:0}, parts));
}

#[derive(Clone, Debug)]
struct Part {
    x: usize,
    y: usize
}

fn get_value(part: Part, parts: Vec<Part>) -> Vec<Part> {
    let get_connectable = parts.clone().into_iter().filter(|ref to_connect| to_connect.x == part.x || to_connect.y == part.y || to_connect.x == part.y || to_connect.y == part.x).collect::<Vec<Part>>();
    println!("{:?}", get_connectable);
    let remaining = parts.clone().into_iter().filter(|other| part.x != other.x && part.y != other.y).collect::<Vec<Part>>();
    println!("{:?}", remaining);
    if get_connectable.len() == 0 {
        println!("{:?}", remaining);
        vec![part]
    } else {
        vec![part]
        .extend(get_connectable
        .iter()
        .map(|part| {
            get_value(part.clone(), remaining.clone())
        .fold(Part {x: 0, y:0}, |max, part| if part.x + part.y > max.x + max.y {
            part
        } else {
            max
        })
        })
    }
}