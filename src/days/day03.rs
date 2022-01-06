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

fn to_i32(slice: &[i32]) -> i32 {
    slice
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, &b): (usize, &i32)| {
            acc + 2_i32.pow(i as u32) * b as i32
        })
}

fn task_one(vec: &Vec<String>) -> i32 {
    let rows = (vec.len() / 2) as i32;
    let length = vec[0].len();
    let mut bits: Vec<i32> = vec![0; length];
    let mut flipped: Vec<i32> = vec![0; length];

    for v in vec {
        for (i, c) in v.chars().enumerate() {
            bits[i] += c.to_digit(10).unwrap() as i32;
        }
    }

    for i in 0..bits.len() {
        bits[i] = if bits[i] > rows { 1 } else { 0 };
        flipped[i] = if bits[i] == 1 { 0 } else { 1 };
    }

    let gamma = to_i32(&bits);
    let epsilon = to_i32(&flipped);

    gamma * epsilon
}

fn task_two(vec: &Vec<String>) -> i32 {
    let gamma = generator_rating(&vec, &|a, b| a >= b);
    let epsilon = generator_rating(&vec, &|a, b| a < b);

    gamma * epsilon
}

fn generator_rating<F>(vec: &Vec<String>, f: &F) -> i32
where
    F: Fn(usize, usize) -> bool,
{
    let mut ones: Vec<String> = Vec::new();
    let mut zeros: Vec<String> = Vec::new();
    let mut res = vec.clone();
    for b in 0..vec[0].len() {
        for v in &res {
            if v.chars().nth(b).unwrap() == '1' {
                ones.push(v.to_string());
            } else {
                zeros.push(v.to_string());
            }
        }
        if ones.len() == 0 || zeros.len() == 0 {
            break;
        };
        res = if f(ones.len(), zeros.len()) {
            ones.clone()
        } else {
            zeros.clone()
        };
        ones.clear();
        zeros.clear();
    }
    i32::from_str_radix(res[0].as_str(), 2).unwrap()
}
