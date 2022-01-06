use aoc::{read_input, time};
use std::collections::*;
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

    let mut m: HashMap<String, Vec<String>> = HashMap::new();
    let vec: Vec<String> = read_input(input);
    for v in vec {
        let (a, b) = v.split_once('-').unwrap();
        if b.to_string() != "start" {
            m.entry(a.to_string())
                .or_insert(Vec::new())
                .push(b.to_string());
        }
        if a.to_string() != "start" {
            m.entry(b.to_string())
                .or_insert(Vec::new())
                .push(a.to_string());
        }
    }
    m.remove("end");
    time("one", task_one, &m);
    time("two", task_two, &m);
}

trait IsLowercase {
    fn is_lowercase(&self) -> bool;
}

impl IsLowercase for String {
    fn is_lowercase(&self) -> bool {
        return &self.to_lowercase() == self;
    }
}

fn walk_1(
    map: &HashMap<String, Vec<String>>,
    city: String,
    visited: &mut HashMap<String, bool>,
    paths: &mut i32,
) {
    if city.to_lowercase() == "end" {
        *paths += 1;
        return;
    }
    // if the city is lowercase and we have visited it already
    if city.is_lowercase() && *visited.get(&city).unwrap() {
        return;
    }
    visited.insert(city.to_string(), true);

    let neighbors = map.get(&city).unwrap();
    for n in neighbors {
        walk_1(map, n.to_string(), visited, paths);
    }
    visited.insert(city.to_string(), false);
}

fn task_one(map: &HashMap<String, Vec<String>>) -> i32 {
    let mut visited: HashMap<String, bool> = HashMap::new();
    for k in map.keys() {
        visited.insert(k.to_string(), false);
    }

    let mut paths = 0;
    for city in map.get("start").unwrap() {
        walk_1(&map, city.to_string(), &mut visited, &mut paths);
        visited.iter_mut().for_each(|x| *x.1 = false);
    }
    paths
}

fn walk_2(
    map: &HashMap<String, Vec<String>>,
    city: String,
    visited: &mut HashMap<String, i32>,
    paths: &mut i32,
) {
    if city.to_lowercase() == "end" {
        *paths += 1;
        return;
    }
    // if the city is lowercase and another city has been visited twice
    // and we already have visited this city before
    if city.is_lowercase()
        && *visited.get(&city.to_string()).unwrap() >= 1
        && visited
            .iter()
            .filter(|x| x.0.is_lowercase() && *x.1 == 2)
            .count()
            > 0
    {
        return;
    }
    *visited.get_mut(&city.to_string()).unwrap() += 1;

    let neighbors = map.get(&city).unwrap();
    for n in neighbors {
        walk_2(map, n.to_string(), visited, paths);
    }
    *visited.get_mut(&city.to_string()).unwrap() -= 1;
}

fn task_two(map: &HashMap<String, Vec<String>>) -> i32 {
    let mut visited: HashMap<String, i32> = HashMap::new();
    for k in map.keys() {
        visited.insert(k.to_string(), 0);
    }

    let mut paths = 0;
    for city in map.get("start").unwrap() {
        walk_2(&map, city.to_string(), &mut visited, &mut paths);
        visited.iter_mut().for_each(|x| *x.1 = 0);
    }
    paths
}
