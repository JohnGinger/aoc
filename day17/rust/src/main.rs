use std::fs::File;
use std::io::Read;

fn main() {
    let file_name = "../input.txt";
    let mut file = File::open(file_name).expect("Unable to open input file!");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Cannot convert file contents to string!");

    let instructions = contents
        .split(",")
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>();
    let mut positions = "abcdefghijklmnop".chars().collect();
    let mut dance_cycle = 0;
    let mut i = 0;
    loop {
        i += 1;
        dance(&mut positions, &instructions);
        if i == 1 {
            println!(
                "Part 1 is {}",
                positions.clone().into_iter().collect::<String>()
            );
        }
        if positions.clone().into_iter().collect::<String>() == "abcdefghijklmnop" {
            dance_cycle = i;
            break;
        }
    }
    let loops = 1000000000 % dance_cycle;
    for _ in 0..loops {
        dance(&mut positions, &instructions)
    }

    println!("Part 2 is {}", positions.into_iter().collect::<String>())
}

fn dance(mut positions: &mut Vec<char>, instructions: &Vec<&str>) {
    for j in 0..instructions.len() {
        let mut dance_move = instructions[j].chars();
        match dance_move.nth(0) {
            Some('s') => {
                let rotation = 16 - dance_move.as_str().parse::<usize>().unwrap();
                rotate(&mut positions, rotation);
            }
            Some('x') => {
                let swap = dance_move
                    .as_str()
                    .split("/")
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<usize>>();
                positions.swap(swap[0], swap[1]);
            }
            Some('p') => {
                let first_char = dance_move.nth(0).unwrap();
                let second_char = dance_move.nth(1).unwrap();
                let first = positions.iter().position(|&x| x == first_char).unwrap();
                let second = positions.iter().position(|&x| x == second_char).unwrap();
                positions.swap(first, second);
            }
            _ => println!("something has gone horribly wrong {}", instructions[j]),
        };
    }
}

fn rotate(vec: &mut Vec<char>, mid: usize) {
    let vec_clone = vec.clone();
    for i in 0..vec.len() {
        vec[i] = vec_clone[(i + mid) % vec.len()];
    }
}
