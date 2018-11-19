use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn main() {
    let file_name = "../input.txt";
    let mut file = File::open(file_name).expect("Unable to open input file!");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Cannot convert file contents to string!");


/*
        parse all possible inputs into outputs for two different sizes e.g. HashMap<vec<char>, vec<char>>


        check size
        for each point replace

    */

    //println!("Part 2 is {}", answer);
}
