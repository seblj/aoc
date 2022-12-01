use aoc::{read_input, time};
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
    time("one", task_one, &vec);
    time("two", task_two, &vec);
}

fn task_one(vec: &Vec<String>) -> i32 {
    let mut depth = 0;
    let mut width = 0;
    for item in vec {
        let (dir, x) = item.split_once(' ').unwrap();
        let x: i32 = x.parse().unwrap();
        if dir == "forward" {
            width += x;
        } else if dir == "up" {
            depth -= x;
        } else if dir == "down" {
            depth += x;
        }
    }
    depth * width
}

fn task_two(vec: &Vec<String>) -> i32 {
    let mut depth = 0;
    let mut width = 0;
    let mut aim = 0;

    for item in vec {
        let (dir, x) = item.split_once(' ').unwrap();
        let x: i32 = x.parse().unwrap();
        if dir == "forward" {
            width += x;
            depth += aim * x;
        } else if dir == "up" {
            aim -= x;
        } else if dir == "down" {
            aim += x;
        }
    }
    depth * width
}
