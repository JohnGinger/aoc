use std::collections::HashMap;

mod knot_hash;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

fn main() {
    let contents = aoc_util::get_input(14);

    let mut found = 0;
    let mut points = HashMap::new();
    let mut region_id = 0;
    let mut joined = 0;
    for y in 0..128 {
        let row = knot_hash::get(format!("{}-{}", contents, y));

        let line_binary = format!(
            "{:?}",
            row.split("")
                .filter(|s| !s.is_empty())
                .map(|x| x.trim())
                .map(|x| format!(
                    "{:04b}",
                    match i32::from_str_radix(x, 16) {
                        Ok(x) => x,
                        Err(e) => panic!("{} could not be parsed ({:?})", x, e),
                    }
                ))
                .collect::<String>()
        );

        let mut x = 0;
        for point in line_binary.split("").filter(|s| !s.is_empty()) {
            x += 1;
            if point == "1" {
                found += 1;

                let point = Point { x, y };

                let point_left = ref_to_value(points.get(&Point { x: x - 1, y }));
                let point_above = ref_to_value(points.get(&Point { x: x, y: y - 1 }));

                match (point_left, point_above) {
                    (Some(left), None) => points.insert(point, left),
                    (None, Some(above)) => points.insert(point, above),
                    (Some(left), Some(above)) => {
                        if left != above {
                            for val in points.values_mut() {
                                if *val == above {
                                    *val = left;
                                }
                            }
                            joined += 1;
                        }
                        points.insert(point, left)
                    }
                    (None, None) => {
                        region_id += 1;
                        points.insert(point, region_id)
                    }
                };
            }
        }
    }
    println!("Part 1 is {}", found);
    println!("Part 2 is {}", region_id - joined);
}

fn ref_to_value(option: Option<&usize>) -> Option<usize> {
    match option {
        Some(value) => Some(*value),
        None => None,
    }
}
