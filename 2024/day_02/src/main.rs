fn is_valid(chars: &[i32]) -> bool {
    let num_pos = chars.windows(2).filter(|it| it[1] - it[0] >= 0).count();
    let all_pos = num_pos >= chars.len() - 2;
    chars.windows(2).all(|c| {
        let at_most_three = c[0].abs_diff(c[1]) <= 3;
        if all_pos {
            at_most_three && c[1] - c[0] > 0
        } else {
            at_most_three && c[0] - c[1] > 0
        }
    })
}

fn task_one(input: &[String]) -> usize {
    input
        .iter()
        .filter(|it| {
            let chars: Vec<i32> = it
                .split_whitespace()
                .map(|c| c.parse::<i32>().unwrap())
                .collect();

            is_valid(&chars)
        })
        .count()
}

fn task_two(input: &[String]) -> usize {
    input
        .iter()
        .filter(|it| {
            let chars: Vec<i32> = it
                .split_whitespace()
                .map(|c| c.parse::<i32>().unwrap())
                .collect();

            for (i, _) in chars.iter().enumerate() {
                let mut cloned = chars.clone();

                if is_valid(&cloned) {
                    return true;
                } else {
                    cloned.remove(i);
                    if is_valid(&cloned) {
                        return true;
                    }
                }
            }

            false
        })
        .count()
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
