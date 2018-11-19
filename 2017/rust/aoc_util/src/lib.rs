use std::io::*;
use std::fs::File;

pub fn get_input(day: i32) -> String {
    let file_name = format!("../../data/{}.txt", day);
    let mut file = File::open(file_name).expect("Unable to open input file!");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Cannot convert file contents to string!");

    String::from(contents.trim())
}

pub fn get_input_lines(day: i32) -> Vec<String> {
    let file_name = format!("../../data/{}.txt", day);
    let file = File::open(file_name).expect("Unable to open input file!");

    BufReader::new(file).lines().map(|l| l.unwrap()).collect()
}

pub struct PuzzleLines {
    lines: Lines<BufReader<File>>,
}

impl PuzzleLines {
    fn new(file_name: String) -> Self {
        let file = File::open(file_name).expect("Unable to open input file!");
        PuzzleLines {
            lines: BufReader::new(file).lines(),
        }
    }
}

impl Iterator for PuzzleLines {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        match self.lines.next() {
            Some(result) => Some(result.expect("could not read input line")),
            None => None,
        }
    }
}

pub fn iterate_input_lines(day: i32) -> PuzzleLines {
    let file_name = format!("../../data/{}.txt", day);
    PuzzleLines::new(file_name)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
