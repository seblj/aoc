use aoc::{read_input, time};
use std::path::Path;

pub fn solve() {
    let input = "test";
    let file = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
    let input = format!(
        "{}/src/days/input/{}.{}",
        env!("CARGO_MANIFEST_DIR"),
        file,
        input
    );

    let vec = read_input(input);
    time("one", task_one, &vec);
    time("two", task_two, &vec);
}

fn task_one(vec: &[i32]) -> i32 {
    unimplemented!()
}

fn task_two(vec: &[i32]) -> i32 {
    unimplemented!()
}
