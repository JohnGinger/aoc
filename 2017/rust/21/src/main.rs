extern crate aoc_util;
use aoc_util::PuzzleLines;
use std::fmt;

#[derive(Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
    value: bool,
}

#[derive(Clone)]
struct Pattern {
    check_points: Vec<Vec<bool>>,
    replace_points: Vec<Vec<bool>>,
}

fn map_pattern(line: &Vec<bool>) -> String {
    format!(
        "{}",
        line.into_iter()
            .map(|&x| if x { "#" } else { "." })
            .collect::<String>()
    )
}

impl fmt::Debug for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut line = format!(
            "\n{} => {}",
            map_pattern(&self.check_points[0]),
            map_pattern(&self.replace_points[0])
        );
        for y in 1..self.check_points[0].len() {
            line = format!(
                "{}\n{}    {}",
                line,
                map_pattern(&self.check_points[y]),
                map_pattern(&self.replace_points[y])
            );
        }
        line = format!(
            "{}\n{}    {}",
            line,
            (&self.check_points[0])
                .into_iter()
                .map(|_| " ")
                .collect::<String>(),
            map_pattern(&self.replace_points[self.replace_points[0].len() - 1])
        );
        write!(f, "{}", line)
    }
}

fn get_patterns(lines: PuzzleLines) -> (Vec<Pattern>, Vec<Pattern>) {
    let mut two = Vec::new();
    let mut three = Vec::new();
    for line in lines {
        if line.len() == 20 {
            two.append(&mut get_pattern(line))
        } else if line.len() == 34 {
            three.append(&mut get_pattern(line))
        }
    }
    (two, three)
}

fn get_pattern(line: String) -> Vec<Pattern> {
    let mut patterns = Vec::new();
    let parts = line.split(" => ").collect::<Vec<&str>>();
    let check_points = get_points(parts[0]);
    let replace_points = get_points(parts[1]);
    let base_pattern = Pattern {
        check_points: check_points,
        replace_points: replace_points,
    };
    let flipped_x = get_flipped_x(&base_pattern);
    let flipped_y = get_flipped_y(&base_pattern);
    let flipped_both = get_flipped_y(&flipped_x);

    patterns.append(&mut get_rotated(&base_pattern));
    patterns.append(&mut get_rotated(&flipped_x));
    patterns.append(&mut get_rotated(&flipped_y));
    patterns.push(base_pattern);
    patterns.push(flipped_x);
    patterns.push(flipped_y);
    patterns.push(flipped_both);

    patterns
}

fn get_flipped_x(pattern: &Pattern) -> Pattern {
    let size = pattern.check_points.len();
    let mut new_pattern = pattern.clone();
    for y in 0..size {
        for x in 0..size {
            new_pattern.check_points[y][x] = pattern.check_points[y][(size - 1 - x) % size]
        }
    }
    new_pattern
}
fn get_flipped_y(pattern: &Pattern) -> Pattern {
    let size = pattern.check_points.len();
    let mut new_pattern = pattern.clone();
    for y in 0..size {
        for x in 0..size {
            new_pattern.check_points[y][x] = pattern.check_points[(size - 1 - y) % size][x]
        }
    }
    new_pattern
}
fn rotate(pattern: &Pattern) -> Pattern {
    let c = &pattern.check_points;
    if c.len() == 2 {
        Pattern {
            check_points: vec![vec![c[1][0], c[0][0]], vec![c[1][1], c[0][1]]],
            replace_points: pattern.replace_points.clone(),
        }
    } else {
        Pattern {
            check_points: vec![
                vec![c[2][0], c[1][0], c[0][0]],
                vec![c[2][1], c[1][1], c[0][1]],
                vec![c[2][2], c[1][2], c[0][2]],
            ],
            replace_points: pattern.replace_points.clone(),
        }
    }
}

fn get_rotated(pattern: &Pattern) -> Vec<Pattern> {
    let mut patterns = Vec::new();
    patterns.push(rotate(pattern));
    patterns.push(rotate(&rotate(pattern)));
    patterns.push(rotate(&rotate(&rotate(pattern))));
    patterns
}

fn get_points(line: &str) -> Vec<Vec<bool>> {
    line.split("/")
        .map(|line| {
            return line.chars().map(|c| return c == '#').collect::<Vec<bool>>();
        })
        .collect()
}

fn divisible_by_two(grid: &Vec<Vec<bool>>) -> bool {
    return grid.len() % 2 == 0;
}

fn pattern_matches(
    grid: &Vec<Vec<bool>>,
    start_x: usize,
    start_y: usize,
    pattern: &Pattern,
) -> bool {
    for y in 0..pattern.check_points.len() {
        for x in 0..pattern.check_points.len() {
            if grid[start_y + y][start_x + x] != pattern.check_points[y][x] {
                return false;
            }
        }
    }
    return true;
}

fn replace_pattern(grid: &mut Vec<Vec<bool>>, start_x: usize, start_y: usize, pattern: &Pattern) {
    let scale = pattern.replace_points.len() as f64 / pattern.check_points.len() as f64;
    let x_offset = (start_x as f64 * scale) as usize;
    let y_offset = (start_y as f64 * scale) as usize;
    for (y, line) in pattern.replace_points.iter().enumerate() {
        for (x, light) in line.iter().enumerate() {
            grid[y + y_offset][x + x_offset] = *light;
        }
    }
}

fn replace_patterns(
    grid: &mut Vec<Vec<bool>>,
    patterns: &Vec<Pattern>,
    size: usize,
) -> Vec<Vec<bool>> {
    let new_size = (grid.len() / size) as usize * (size + 1);
    let mut new_grid = vec![vec![false; new_size]; new_size];
    let range = (0..grid.len()).step_by(size);
    for y in range.clone() {
        for x in range.clone() {
            for pattern in patterns {
                if pattern_matches(&grid, x, y, &pattern) {
                    //println!("match with {:?}", &pattern);
                    replace_pattern(&mut new_grid, x, y, &pattern);
                    break;
                }
            }
        }
    }
    new_grid
}

fn main() {
    let lines = aoc_util::iterate_input_lines(21);
    let (two_patterns, three_patterns) = get_patterns(lines);

    println!(
        "Part 1 is {}",
        get_on_after_iterations(&two_patterns, &three_patterns, 5)
    );
    println!(
        "Part 2 is {}",
        get_on_after_iterations(&two_patterns, &three_patterns, 18)
    );
}

fn get_on_after_iterations(
    two_patterns: &Vec<Pattern>,
    three_patterns: &Vec<Pattern>,
    iterations: usize,
) -> usize {
    let mut grid = vec![
        vec![false, true, false],
        vec![false, false, true],
        vec![true, true, true],
    ];

    let mut i = 0;
    while i < iterations {
        if divisible_by_two(&grid) {
            grid = replace_patterns(&mut grid, two_patterns, 2)
        } else {
            grid = replace_patterns(&mut grid, three_patterns, 3)
        }
        i += 1;
        if i < 5 {
            display_grid(&grid)
        }
    }

    grid.into_iter()
        .flat_map(|line| line.into_iter().map(|point| if point { 1 } else { 0 }))
        .sum()
}

fn display_grid(grid: &Vec<Vec<bool>>) {
    for line in grid {
        println!(
            "{}",
            line.into_iter()
                .map(|&x| if x { "#" } else { "." })
                .collect::<String>()
        )
    }
    println!("");
}
