use aoc::{read_input, time};
use std::collections::*;
use std::path::Path;

pub fn solve() {
    let input = "input";
    let file = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
    let input = format!(
        "{}/src/days/input/{}.{}",
        env!("CARGO_MANIFEST_DIR"),
        file,
        input
    );

    let vec: Vec<String> = read_input(input);
    let vec: Vec<Vec<u32>> = vec
        .iter()
        .map(|v| v.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    time("one", task_one, &vec);
    time("two", task_two, &vec);
}

fn is_low_point(vec: &Vec<Vec<u32>>, i: usize, j: usize) -> bool {
    let max_i = vec.len() - 1;
    let max_j = vec[0].len() - 1;
    let num = vec[i][j];

    if j < max_j && vec[i][j + 1] <= num {
        return false;
    }
    if j > 0 && vec[i][j - 1] <= num {
        return false;
    }
    if i < max_i && vec[i + 1][j] <= num {
        return false;
    }
    if i > 0 && vec[i - 1][j] <= num {
        return false;
    }

    return true;
}

fn task_one(vec: &Vec<Vec<u32>>) -> u32 {
    let mut result = 0;
    for (i, v) in vec.iter().enumerate() {
        for (j, _) in v.iter().enumerate() {
            if is_low_point(&vec, i, j) {
                result += vec[i][j] + 1;
            }
        }
    }
    result
}

#[derive(Eq, PartialEq)]
struct Point {
    i: i32,
    j: i32,
}

impl Point {
    fn new(i: i32, j: i32) -> Self {
        return Self { i, j };
    }

    /// Returns a tuple of all directions from point (i, j)
    /// (down, right, up, left)
    fn get_sides(i: i32, j: i32) -> (Self, Self, Self, Self) {
        return (
            Point { i: i - 1, j },
            Point { i, j: j + 1 },
            Point { i: i + 1, j },
            Point { i, j: j - 1 },
        );
    }
}

fn find_basin(
    vec: &Vec<Vec<u32>>,
    seen: &mut HashSet<(i32, i32)>,
    point: &Point,
    prev: u32,
    start: &Point,
) -> bool {
    let i = point.i;
    let j = point.j;

    // If out of bounds
    if i < 0 || j < 0 || i as usize >= vec.len() || j as usize >= vec[0].len() {
        return false;
    }

    let curr = vec[i as usize][j as usize];
    if curr == 9 || (prev >= curr && point != start) || !seen.insert((i, j)) {
        return false;
    }

    // Recurse further to find elements in the basin
    let (down, right, up, left) = Point::get_sides(i, j);
    return find_basin(vec, seen, &down, curr, start)
        || find_basin(vec, seen, &right, curr, start)
        || find_basin(vec, seen, &up, curr, start)
        || find_basin(vec, seen, &left, curr, start);
}

fn task_two(vec: &Vec<Vec<u32>>) -> usize {
    let mut result: Vec<usize> = Vec::new();
    for (i, v) in vec.iter().enumerate() {
        for (j, _) in v.iter().enumerate() {
            if is_low_point(&vec, i, j) {
                let mut seen: HashSet<(i32, i32)> = HashSet::new();
                let point = Point::new(i as i32, j as i32);
                find_basin(&vec, &mut seen, &point, vec[i][j], &point);
                result.push(seen.len());
            }
        }
    }
    result.sort_unstable();
    result.iter().rev().take(3).product()
}
