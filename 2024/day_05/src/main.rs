use std::collections::{HashMap, HashSet};

#[derive(Default)]
struct Page {
    before: HashMap<i32, HashSet<i32>>,
    after: HashMap<i32, HashSet<i32>>,
    updates: Vec<Vec<i32>>,
}

fn parse(input: &[String]) -> Page {
    let mut page = Page::default();

    for line in input {
        if let Some((left, right)) = line.split_once('|') {
            let left = left.parse::<i32>().unwrap();
            let right = right.parse::<i32>().unwrap();

            page.before.entry(left).or_default().insert(right);

            page.after.entry(right).or_default().insert(left);
        } else {
            let numbers = line
                .split(',')
                .flat_map(|num| num.parse::<i32>())
                .collect::<Vec<_>>();

            if !numbers.is_empty() {
                page.updates.push(numbers);
            }
        }
    }

    page
}

fn get_updates(page: &Page) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut valid_updates = vec![];
    let mut invalid_updates = vec![];

    for update in page.updates.iter() {
        let mut valid = true;
        for (i, num) in update.iter().enumerate() {
            let (before, after) = update.split_at(i);
            let before_set = HashSet::from_iter(before.iter().map(|it| it.to_owned()));
            let after_set = HashSet::from_iter(after.iter().map(|it| it.to_owned()));

            if let Some(allowed_before) = page.before.get(num) {
                if !allowed_before
                    .intersection(&before_set)
                    .collect::<Vec<_>>()
                    .is_empty()
                {
                    valid = false;
                    continue;
                }
            }

            if let Some(allowed_after) = page.after.get(num) {
                if !allowed_after
                    .intersection(&after_set)
                    .collect::<Vec<_>>()
                    .is_empty()
                {
                    valid = false;
                    continue;
                }
            }
        }
        if valid {
            valid_updates.push(update.to_vec());
        } else {
            invalid_updates.push(update.to_vec());
        }
    }

    (valid_updates, invalid_updates)
}

fn task_one(input: &[String]) -> usize {
    let page = parse(input);
    let (valid_updates, _) = get_updates(&page);

    valid_updates
        .iter()
        .map(|it| it[(it.len() - 1) / 2] as usize)
        .sum()
}

fn task_two(input: &[String]) -> usize {
    let page = parse(input);
    let (_, invalid_updates) = get_updates(&page);

    invalid_updates
        .into_iter()
        .map(|it| {
            let mut vec: Vec<i32> = vec![];
            for num in it.iter() {
                let empty = HashSet::new();
                let after = page.after.get(num).unwrap_or(&empty);

                let index = vec
                    .iter()
                    .position(|it| after.contains(it))
                    .unwrap_or(vec.len());

                vec.insert(index, *num);
            }
            vec
        })
        .map(|it| it[(it.len() - 1) / 2] as usize)
        .sum()
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
