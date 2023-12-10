use std::{cmp::Ordering, iter::Peekable, slice::Iter};

#[derive(Debug, Eq, PartialEq, Clone)]
enum Item {
    List(Vec<Item>),
    Number(i32),
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Item::List(a), Item::Number(b)) => a.cmp(&vec![Item::Number(*b)]),
            (Item::Number(a), Item::List(b)) => vec![Item::Number(*a)].cmp(b),
            (Item::Number(a), Item::Number(b)) => a.cmp(b),
            (Item::List(a), Item::List(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_number(iter: &mut Peekable<Iter<char>>, c: &char) -> Item {
    let mut vec = vec![c];
    while let Some(d) = iter.peek() {
        match d {
            ',' => {
                iter.next();
                break;
            }
            '[' | ']' => break,
            num => {
                vec.push(num);
                iter.next();
            }
        }
    }
    let num = vec.into_iter().collect::<String>().parse::<i32>().unwrap();
    Item::Number(num)
}

fn parse_list(iter: &mut Peekable<Iter<char>>) -> Vec<Item> {
    let mut vec = Vec::new();
    while let Some(c) = iter.next() {
        match c {
            '[' => vec.push(Item::List(parse_list(iter))),
            ']' => return vec,
            ',' => continue,
            num => vec.push(parse_number(iter, num)),
        }
    }
    vec
}

fn parse_packet(s: &str) -> Vec<Item> {
    let chars: Vec<char> = s
        .trim()
        .strip_prefix('[')
        .unwrap()
        .strip_suffix(']')
        .unwrap()
        .chars()
        .collect();

    let mut v = Vec::new();
    let mut iter = chars.iter().peekable();

    while let Some(c) = iter.next() {
        match c {
            ',' => continue,
            '[' => v.push(Item::List(parse_list(&mut iter))),
            num => v.push(parse_number(&mut iter, num)),
        }
    }

    v
}

fn task_one(input: &[String]) -> usize {
    let v: Vec<(String, String)> = input
        .split(|s| s.is_empty())
        .map(|v| (v[0].clone(), v[1].clone()))
        .collect();

    let mut correct = 0;
    v.into_iter().enumerate().for_each(|(idx, (left, right))| {
        let left = parse_packet(&left);
        let right = parse_packet(&right);

        match left.cmp(&right) {
            Ordering::Less => correct += idx + 1,
            Ordering::Greater => {}
            Ordering::Equal => {}
        }
    });
    correct
}

fn task_two(input: &[String]) -> usize {
    let v: Vec<(String, String)> = input
        .split(|s| s.is_empty())
        .map(|v| (v[0].clone(), v[1].clone()))
        .collect();

    let divider_1 = parse_packet("[[2]]");
    let divider_2 = parse_packet("[[6]]");

    let mut vec = vec![divider_1.clone(), divider_2.clone()];

    v.into_iter().for_each(|(left, right)| {
        vec.push(parse_packet(&left));
        vec.push(parse_packet(&right));
    });

    vec.sort();

    vec.into_iter().enumerate().fold(1, |acc, (idx, item)| {
        if item == divider_1 || item == divider_2 {
            acc * (idx + 1)
        } else {
            acc
        }
    })
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
    let elapsed = t.elapsed().as_millis();

    match task {
        Task::One => {
            println!("({}ms)\tTask one: \x1b[0;34;34m{}\x1b[0m", elapsed, res);
        }
        Task::Two => {
            println!("({}ms)\tTask two: \x1b[0;33;10m{}\x1b[0m", elapsed, res);
        }
    };
}

fn get_input_file() -> String {
    std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input".to_string())
}
