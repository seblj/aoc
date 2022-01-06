use aoc::{read_input, time};
use std::collections::*;
use std::path::Path;

#[derive(Debug)]
struct Map {
    template: Vec<char>,
    pairs: HashMap<String, char>,
}
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
    let mut pairs: HashMap<String, char> = HashMap::new();
    vec.iter().skip(2).for_each(|x| {
        let (k, v) = x.split_once(" -> ").unwrap();
        pairs.insert(k.to_string(), v.parse::<char>().unwrap());
    });

    let template: Vec<char> = vec.first().unwrap().chars().collect();
    let map = Map { pairs, template };

    time("one", task_one, &map);
    time("two", task_two, &map);
}

fn get_starting_frequency(template: &Vec<char>) -> HashMap<char, i64> {
    let mut counter: HashMap<char, i64> = HashMap::new();
    template.iter().for_each(|&x| {
        let count = counter.entry(x).or_insert(0);
        *count += 1;
    });
    return counter;
}

fn get_start_frequency(template: &Vec<char>) -> HashMap<String, i64> {
    let mut counter: HashMap<String, i64> = HashMap::new();
    template.windows(2).for_each(|x| {
        let string: String = x.iter().collect();
        let count = counter.entry(string).or_insert(0);
        *count += 1;
    });
    return counter;
}

fn task_one(map: &Map) -> i64 {
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
    let max = frequency.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let min = frequency.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    max - min
}

fn task_two(map: &Map) -> i64 {
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

    let mut res_map: HashMap<char, i64> = HashMap::new();
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
        let a = a / 2 as f64;
        let a = a.ceil();
        *res = a as i64;
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

    let max = res_map.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let min = res_map.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    max - min
}
