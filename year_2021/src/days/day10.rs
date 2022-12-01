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

    let vec = read_input(input);
    time("one", task_one, &vec);
    time("two", task_two, &vec);
}

enum LineStatus {
    Corrupted(char),
    Incomplete(Vec<char>),
}

// Implement FromStr trait so we can parse the vec of strings
// into the enum LineStatus
impl std::str::FromStr for LineStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = Vec::new();
        for ch in s.chars() {
            match ch {
                '(' | '[' | '{' | '<' => stack.push(ch),
                _ => match stack.pop() {
                    Some(c) => {
                        if get_matching(c) != ch {
                            return Ok(LineStatus::Corrupted(ch));
                        }
                    }
                    None => return Ok(LineStatus::Corrupted(ch)),
                },
            }
        }
        Ok(LineStatus::Incomplete(stack))
    }
}

fn get_matching(opening: char) -> char {
    match opening {
        '{' => '}',
        '[' => ']',
        '<' => '>',
        '(' => ')',
        _ => unreachable!(),
    }
}

fn score(c: char) -> i32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

fn score2(c: char) -> i64 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => unreachable!(),
    }
}

fn task_one(vec: &[String]) -> i32 {
    vec.iter()
        .filter_map(|line| match line.parse::<LineStatus>().unwrap() {
            LineStatus::Corrupted(ch) => Some(score(ch)),
            _ => None,
        })
        .sum()
}

fn task_two(vec: &[String]) -> i64 {
    let mut vec: Vec<_> = vec
        .iter()
        .filter_map(|line| match line.parse::<LineStatus>().unwrap() {
            LineStatus::Incomplete(stack) => Some(
                stack
                    .into_iter()
                    .rev()
                    .fold(0, |acc, ch| (acc * 5) + score2(ch)),
            ),
            _ => None,
        })
        .collect();
    vec.sort_unstable();
    vec[vec.len() / 2]
}
