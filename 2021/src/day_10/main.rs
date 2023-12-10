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

fn score(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

fn score2(c: char) -> usize {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => unreachable!(),
    }
}

fn task_one(input: &[String]) -> usize {
    input
        .iter()
        .filter_map(|line| match line.parse::<LineStatus>().unwrap() {
            LineStatus::Corrupted(ch) => Some(score(ch)),
            _ => None,
        })
        .sum()
}

fn task_two(input: &[String]) -> usize {
    let mut vec: Vec<_> = input
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
