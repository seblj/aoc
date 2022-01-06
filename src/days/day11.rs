use aoc::{read_input, time};
use std::path::Path;

#[derive(Clone)]
struct Octopus {
    energy: u32,
    visited: bool,
}

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
    let vec: Vec<Vec<Octopus>> = vec
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| Octopus {
                    energy: c.to_digit(10).unwrap(),
                    visited: false,
                })
                .collect()
        })
        .collect();
    time("one", task_one, vec.clone());
    time("two", task_two, vec.clone());
}

fn update(vec: &mut Vec<Vec<Octopus>>, i: i32, j: i32) {
    if i < 0 || j < 0 || i >= vec.len() as i32 || j >= vec[0].len() as i32 {
        return;
    }
    let i = i as usize;
    let j = j as usize;
    vec[i][j].energy += 1;
    if vec[i][j].energy > 9 && !vec[i][j].visited {
        vec[i][j].visited = true;
        update_surroundings(vec, i, j);
    }
}

fn update_surroundings(vec: &mut Vec<Vec<Octopus>>, i: usize, j: usize) {
    let i = i as i32;
    let j = j as i32;
    update(vec, i - 1, j + 1); // upper right corner
    update(vec, i, j + 1); // right
    update(vec, i + 1, j + 1); // lower right corner
    update(vec, i + 1, j - 1); // lower left corner
    update(vec, i, j - 1); // left
    update(vec, i - 1, j - 1); // upper left corner
    update(vec, i - 1, j); // up
    update(vec, i + 1, j); // down
}

fn task_one(mut vec: Vec<Vec<Octopus>>) -> i32 {
    let mut flashes = 0;
    for _ in 0..100 {
        for i in 0..vec.len() {
            for j in 0..vec[0].len() {
                update(&mut vec, i as i32, j as i32);
            }
        }
        for i in 0..vec.len() {
            for j in 0..vec[0].len() {
                if vec[i][j].energy > 9 {
                    vec[i][j].energy = 0;
                    vec[i][j].visited = false;
                    flashes += 1;
                }
            }
        }
    }
    flashes as i32
}

fn task_two(mut vec: Vec<Vec<Octopus>>) -> i32 {
    for step in 1.. {
        for i in 0..vec.len() {
            for j in 0..vec[0].len() {
                update(&mut vec, i as i32, j as i32);
            }
        }
        // If visited all
        if vec.iter().all(|x| x.iter().all(|c| c.visited)) {
            return step;
        }
        for i in 0..vec.len() {
            for j in 0..vec[0].len() {
                if vec[i][j].energy > 9 {
                    vec[i][j].energy = 0;
                    vec[i][j].visited = false;
                }
            }
        }
    }
    unreachable!();
}
