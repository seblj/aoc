use std::collections::HashMap;

#[derive(Debug)]
struct Map {
    template: Vec<char>,
    pairs: HashMap<String, char>,
}

fn get_starting_frequency(template: &[char]) -> HashMap<char, usize> {
    let mut counter: HashMap<char, usize> = HashMap::new();
    template.iter().for_each(|&x| {
        let count = counter.entry(x).or_insert(0);
        *count += 1;
    });
    counter
}

fn get_start_frequency(template: &[char]) -> HashMap<String, usize> {
    let mut counter: HashMap<String, usize> = HashMap::new();
    template.windows(2).for_each(|x| {
        let string: String = x.iter().collect();
        let count = counter.entry(string).or_insert(0);
        *count += 1;
    });
    counter
}

fn parse(input: &[String]) -> Map {
    let mut pairs: HashMap<String, char> = HashMap::new();
    input.iter().skip(2).for_each(|x| {
        let (k, v) = x.split_once(" -> ").unwrap();
        pairs.insert(k.to_string(), v.parse::<char>().unwrap());
    });

    let template: Vec<char> = input.first().unwrap().chars().collect();
    Map { pairs, template }
}

fn task_one(input: &[String]) -> usize {
    let map = parse(input);
    let mut template = map.template.clone();
    let mut frequency = get_starting_frequency(&template);
    for _ in 0..10 {
        let mut next = template.clone();
        template.windows(2).enumerate().for_each(|(i, x)| {
            let chars: String = x.iter().collect();
            let char = map.pairs.get(&chars).unwrap();

            let count = frequency.entry(*char).or_insert(0);
            *count += 1;
            next.insert(2 * i + 1, *char);
        });
        template = next;
    }
    let max = frequency.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().1;
    let min = frequency.iter().min_by(|a, b| a.1.cmp(b.1)).unwrap().1;
    max - min
}

fn task_two(input: &[String]) -> usize {
    let map = parse(input);
    let template = map.template.clone();
    let mut frequency = get_start_frequency(&template);
    for _ in 0..40 {
        let mut next = frequency.clone();
        frequency.iter().for_each(|(k, v)| {
            let char = *map.pairs.get(k).unwrap();
            let (c1, c2) = k.split_at(1);
            let pair1 = format!("{}{}", c1, char);
            let pair2 = format!("{}{}", char, c2);

            let existing = next.entry(k.to_string()).or_default();
            if *existing == *v {
                next.remove_entry(k);
            } else {
                *existing -= v;
            }

            let count1 = next.entry(pair1).or_insert(0);
            *count1 += v;

            let count2 = next.entry(pair2).or_insert(0);
            *count2 += v;
        });
        frequency = next;
    }

    let mut res_map: HashMap<char, usize> = HashMap::new();
    for (k, v) in &frequency {
        let (c1, c2) = k.split_at(1);
        let c1 = c1.parse::<char>().unwrap();
        let c2 = c2.parse::<char>().unwrap();
        if c1 == c2 {
            continue;
        }
        let res1 = res_map.entry(c1).or_insert(0);
        *res1 += v;

        let res2 = res_map.entry(c2).or_insert(0);
        *res2 += v;
    }

    let a = res_map.clone();
    for (k, _) in a {
        let res = res_map.entry(k).or_default();
        let a = *res as f64;
        let a = a / 2_f64;
        let a = a.ceil();
        *res = a as usize;
    }

    for (k, v) in &frequency {
        let (c1, c2) = k.split_at(1);
        let c1 = c1.parse::<char>().unwrap();
        let c2 = c2.parse::<char>().unwrap();
        if c1 != c2 {
            continue;
        }
        let res1 = res_map.entry(c1).or_insert(0);
        *res1 += v;
    }

    let max = res_map.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().1;
    let min = res_map.iter().min_by(|a, b| a.1.cmp(b.1)).unwrap().1;
    max - min
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
