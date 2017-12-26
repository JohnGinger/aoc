use std::fs::File;
use std::io::Read;

mod knot_hash;

fn main() {
    let file_name = "../input.txt";
    let mut file = File::open(file_name).expect("Unable to open input file!");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Cannot convert file contents to string!");

    let mut found = 0;
    for row in 0..128 {
        let row = knot_hash::get(format!("{}-{}", contents, row));

        let line_binary = format!(
            "{:?}",
            row.split("")
                .filter(|s| !s.is_empty())
                .map(|x| x.trim())
                .map(|x| {
                    format!(
                        "{:04b}",
                        match i32::from_str_radix(x, 16) {
                            Ok(x) => x,
                            Err(e) => panic!("{} could not be parsed ({:?})", x, e),
                        }
                    )
                })
                .collect::<String>()
        );



        found += line_binary
            .split("")
            .filter(|s| !s.is_empty())
            .filter(|&x| x == "1")
            .fold(0, |acc, _c| acc + 1);
    }
    println!("Part 1 is {}", found);
}
