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
    let vec: Vec<i32> = vec
        .first()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    time("one", task_one, &vec);
    time("two", task_two, &vec);
}

fn median(mut vec: Vec<i32>) -> i32 {
    vec.sort();
    let mid = vec.len() / 2;
    vec[mid]
}

fn mean(vec: &[i32]) -> f32 {
    vec.iter().sum::<i32>() as f32 / vec.len() as f32
}

fn divergent(num: i32) -> i32 {
    (num * (num + 1)) / 2
}

fn task_one(vec: &[i32]) -> i32 {
    let med = median(vec.to_vec());
    let mut sum = 0;
    vec.iter().for_each(|&x| sum += (med - x).abs());
    sum
}

fn task_two(vec: &[i32]) -> i32 {
    let mean_ceil = mean(&vec).ceil() as i32;
    let mean_floor = mean(&vec).floor() as i32;

    let mut sum_ceil = 0;
    let mut sum_floor = 0;

    vec.iter().for_each(|&x| {
        sum_floor += divergent((mean_floor - x).abs());
        sum_ceil += divergent((mean_ceil - x).abs());
    });

    std::cmp::min(sum_ceil, sum_floor)
}
