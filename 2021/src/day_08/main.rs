use std::{
    collections::HashSet,
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone)]
struct Entry {
    pattern: Vec<String>,
    output: Vec<String>,
}

#[derive(Debug, Clone)]
struct Numbers {
    zero: HashSet<char>,
    one: HashSet<char>,
    two: HashSet<char>,
    three: HashSet<char>,
    four: HashSet<char>,
    five: HashSet<char>,
    six: HashSet<char>,
    seven: HashSet<char>,
    eight: HashSet<char>,
    nine: HashSet<char>,
}

impl Numbers {
    fn new() -> Self {
        Self {
            zero: HashSet::new(),
            one: HashSet::new(),
            two: HashSet::new(),
            three: HashSet::new(),
            four: HashSet::new(),
            five: HashSet::new(),
            six: HashSet::new(),
            seven: HashSet::new(),
            eight: HashSet::new(),
            nine: HashSet::new(),
        }
    }
}

impl Index<i32> for Numbers {
    type Output = HashSet<char>;
    fn index(&self, idx: i32) -> &Self::Output {
        match idx {
            0 => &self.zero,
            1 => &self.one,
            2 => &self.two,
            3 => &self.three,
            4 => &self.four,
            5 => &self.five,
            6 => &self.six,
            7 => &self.seven,
            8 => &self.eight,
            9 => &self.nine,
            _ => unreachable!(),
        }
    }
}

impl IndexMut<i32> for Numbers {
    fn index_mut(&mut self, idx: i32) -> &mut Self::Output {
        match idx {
            0 => &mut self.zero,
            1 => &mut self.one,
            2 => &mut self.two,
            3 => &mut self.three,
            4 => &mut self.four,
            5 => &mut self.five,
            6 => &mut self.six,
            7 => &mut self.seven,
            8 => &mut self.eight,
            9 => &mut self.nine,
            _ => unreachable!(),
        }
    }
}

fn is_unique(length: usize) -> bool {
    length == 2 || length == 4 || length == 3 || length == 7
}

// Add the easy numbers to the number struct
fn collect_known(numbers: &mut Numbers, patterns: &Vec<String>) {
    for pattern in patterns {
        match pattern.len() {
            2 => numbers.one = pattern.chars().collect(),
            3 => numbers.seven = pattern.chars().collect(),
            4 => numbers.four = pattern.chars().collect(),
            7 => numbers.eight = pattern.chars().collect(),
            _ => {}
        }
    }
}

// Calculate if the pattern is either a 2, 3 or 5
fn calculate_two_three_or_five(numbers: &mut Numbers, pattern: String) {
    // 3 is the only one that contains all the same as 1
    if numbers.one.iter().all(|&c| pattern.contains(c)) {
        numbers.three = pattern.chars().collect();
    } else {
        // If the number of different chars between the pattern
        // and the characters in four is 3, then we have found
        // the pattern for 2. Else it is the pattern for 3.
        let pat: HashSet<char> = pattern.chars().collect();
        if pat.difference(&numbers.four).collect::<Vec<_>>().len() == 3 {
            numbers.two = pattern.chars().collect();
        } else {
            numbers.five = pattern.chars().collect();
        }
    }
}

// Calculate the numbers 0, 6 or 9
fn calculate_zero_six_or_nine(numbers: &mut Numbers, pattern: String) {
    // 6 is the only one that doesn't contain all the same
    // characters that are in 1
    if !numbers.one.iter().all(|&c| pattern.contains(c)) {
        numbers.six = pattern.chars().collect();
    } else {
        // Zero does not contain all the characters that 4 have
        if !numbers.four.iter().all(|&c| pattern.contains(c)) {
            numbers.zero = pattern.chars().collect();
        } else {
            numbers.nine = pattern.chars().collect();
        }
    }
}

// Iterate over the output and calculate what number the output is
fn calculate_output(numbers: &Numbers, output: &Vec<String>) -> String {
    let mut str = "".to_string();
    for out in output {
        let num: HashSet<char> = out.chars().collect();
        for i in 0..10 {
            if numbers[i].eq(&num) {
                str.push_str(&i.to_string());
            }
        }
    }
    str
}

fn parse(input: &[String]) -> Vec<Entry> {
    input
        .iter()
        .map(|x| {
            let (pattern, output) = x.split_once('|').unwrap();
            return Entry {
                pattern: pattern.split_whitespace().map(|p| p.to_string()).collect(),
                output: output.split_whitespace().map(|o| o.to_string()).collect(),
            };
        })
        .collect()
}

fn task_one(input: &[String]) -> usize {
    let vec = parse(input);
    let mut unique = 0;
    for entry in &vec {
        for output in &entry.output {
            if is_unique(output.len()) {
                unique += 1;
            }
        }
    }
    unique
}

fn task_two(input: &[String]) -> usize {
    let vec = parse(input);

    let mut res: Vec<usize> = Vec::new();
    let mut numbers = Numbers::new();
    for entry in vec {
        collect_known(&mut numbers, &entry.pattern);
        for pattern in &entry.pattern {
            if pattern.len() == 6 {
                calculate_zero_six_or_nine(&mut numbers, pattern.to_string());
            } else if pattern.len() == 5 {
                calculate_two_three_or_five(&mut numbers, pattern.to_string());
            }
        }
        let output = calculate_output(&numbers, &entry.output);
        res.push(output.parse::<usize>().unwrap());
    }
    res.iter().sum()
}

fn main() {
    let input = read_input(get_input_file());
    time(Task::One, task_one, &input);
    time(Task::Two, task_two, &input);
}

fn read_input<P>(path: P) -> Vec<String>
where
    P: AsRef<std::path::Path>,
{
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

enum Task {
    One,
    Two,
}

fn time<F, T, U>(task: Task, f: F, arg: T)
where
    F: Fn(T) -> U,
    U: std::fmt::Display,
{
    let t = std::time::Instant::now();
    let res = f(arg);
    let elapsed = t.elapsed();
    let fmt = std::env::var("TASKUNIT").unwrap_or("ms".to_owned());

    let (u, elapsed) = match fmt.as_str() {
        "ms" => ("ms", elapsed.as_millis()),
        "ns" => ("ns", elapsed.as_nanos()),
        "us" => ("μs", elapsed.as_micros()),
        "s" => ("s", elapsed.as_secs() as u128),
        _ => panic!("unsupported time format"),
    };

    match task {
        Task::One => {
            println!("({}{u})\tTask one: \x1b[0;34;34m{}\x1b[0m", elapsed, res);
        }
        Task::Two => {
            println!("({}{u})\tTask two: \x1b[0;33;10m{}\x1b[0m", elapsed, res);
        }
    };
}

fn get_input_file() -> String {
    std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input".to_string())
}
