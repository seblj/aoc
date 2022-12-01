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

fn task_one(vec: &[i32]) -> usize {
    calculate_population(vec, 80)
}

fn task_two(vec: &[i32]) -> usize {
    calculate_population(vec, 256)
}

fn transform_input(vec: &[i32]) -> [usize; 9] {
    let mut array: [usize; 9] = [0; 9];

    for val in vec {
        array[*val as usize] += 1;
    }
    array
}

fn calculate_population(vec: &[i32], days: usize) -> usize {
    let mut arr = transform_input(&vec);
    for _ in 0..days {
        arr.rotate_left(1);
        arr[6] += arr[8];
    }
    arr.iter().sum()
}
