use std::collections::HashMap;
extern crate aoc_util;
#[derive(PartialEq, Eq, Hash, Debug)]
struct Coord(i32, i32);

fn main() {
    let target_number: i32 = aoc_util::get_input(3).parse().unwrap();
    println!("Target is {}", target_number);

    let mut x = 0;
    let mut y = 0;
    let mut level = 0;
    let mut corner = 1;
    let mut index: i32 = 2;
    let mut grid = HashMap::new();
    let mut part_two_printed = false;
    grid.insert(Coord(0, 0), 1);
    while index <= target_number {
        if index > corner {
            level += 1;
            corner += 8 * level;
            x += 1;
        } else if x == level && y < level {
            y += 1;
        } else if x == -level && y > -level {
            y -= 1;
        } else if y == level && x > -level {
            x -= 1;
        } else if y == -level && x < level {
            x += 1;
        }
        if !part_two_printed {
            let new_value = grid.get(&Coord(x - 1, y)).unwrap_or(&0)
                + grid.get(&Coord(x - 1, y - 1)).unwrap_or(&0)
                + grid.get(&Coord(x, y - 1)).unwrap_or(&0)
                + grid.get(&Coord(x + 1, y)).unwrap_or(&0)
                + grid.get(&Coord(x, y + 1)).unwrap_or(&0)
                + grid.get(&Coord(x + 1, y + 1)).unwrap_or(&0)
                + grid.get(&Coord(x - 1, y + 1)).unwrap_or(&0)
                + grid.get(&Coord(x + 1, y - 1)).unwrap_or(&0);

            if new_value > target_number {
                part_two_printed = true;
                println!("Part 2 solution is {}", new_value)
            }
            grid.insert(Coord(x, y), new_value);
        }


        if index == target_number {
            println!("Part 1 solution is {}", i32::abs(x) + i32::abs(y));
        }
        index += 1;
    }
}
