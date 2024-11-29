use std::{collections::HashSet, str::FromStr};

use itertools::Itertools;

#[derive(Debug)]
struct Sensor {
    pos: (i32, i32),
    beacon: (i32, i32),
}

trait Distance {
    fn distance(&self, other: (i32, i32)) -> i32;
}

impl Distance for (i32, i32) {
    fn distance(&self, other: (i32, i32)) -> i32 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

impl FromStr for Sensor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (sensor, beacon) = s.split(':').collect_tuple().unwrap();

        let (x, y) = sensor.split(',').collect_tuple().unwrap();
        let x = x.split('=').find_map(|s| s.parse::<i32>().ok()).unwrap();
        let y = y.split('=').find_map(|s| s.parse::<i32>().ok()).unwrap();

        let sensor = (x, y);

        let (x, y) = beacon.split(',').collect_tuple().unwrap();
        let x = x.split('=').find_map(|s| s.parse::<i32>().ok()).unwrap();
        let y = y.split('=').find_map(|s| s.parse::<i32>().ok()).unwrap();

        let beacon = (x, y);

        Ok(Self {
            pos: sensor,
            beacon,
        })
    }
}

fn task_one(input: &[String]) -> i32 {
    let line = 2000000;
    let mut beacon_set = HashSet::new();
    let mut vec: Vec<Range<i32>> = Vec::new();
    input.iter().for_each(|s| {
        let sensor = Sensor::from_str(s).unwrap();
        beacon_set.insert(sensor.beacon);
        let diff = sensor.pos.distance(sensor.beacon) - (line - sensor.pos.1).abs();
        let mut list = [sensor.pos.0 - diff, sensor.pos.0 + diff];
        list.sort();

        vec.push(list[0]..list[1] + 1);
    });

    vec.sort_by(|left, right| left.start.cmp(&right.start));
    let merged: Vec<_> = merge_ranges(vec).collect();
    let mut num_beacons = 0;
    beacon_set.iter().for_each(|point| {
        if line == point.1 && merged[0].contains(&point.0) {
            num_beacons += 1;
        }
    });

    merged[0].end - merged[0].start - num_beacons
}

fn task_two(input: &[String]) -> i64 {
    let max = 4000000;
    for line in 0..=max {
        let mut vec: Vec<Range<i32>> = Vec::new();
        let mut beacon_set = HashSet::new();
        for s in input.iter() {
            let sensor = Sensor::from_str(s).unwrap();
            beacon_set.insert(sensor.beacon);
            let distance = sensor.pos.distance(sensor.beacon);

            let diff = distance - (line - sensor.pos.1).abs();
            if diff < 0 {
                continue;
            }

            let mut list = [sensor.pos.0 - diff, sensor.pos.0 + diff];
            list.sort();
            let mut start = list[0];
            let mut end = list[1];

            if start < 0 && end < 0 {
                break;
            } else if start < 0 {
                start = 0;
            }
            if start > max && end > max {
                break;
            } else if end > max {
                end = max;
            }

            vec.push(start..end + 1);
        }

        vec.sort_by(|left, right| left.start.cmp(&right.start));
        let merged: Vec<_> = merge_ranges(vec).collect();
        if merged.len() == 2 {
            let y = line;
            let x = merged[0].end;
            return (x as i64 * 4000000) + y as i64;
        }
    }

    unreachable!();
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

// https://codereview.stackexchange.com/questions/103864/merging-an-overlapping-collection-of-intervals
use std::cmp;
use std::ops::Range;

struct MergedRanges<I> {
    values: I,
    last: Option<Range<i32>>,
}

fn merge_ranges<I>(iterator: I) -> MergedRanges<I::IntoIter>
where
    I: IntoIterator<Item = Range<i32>>,
{
    let mut iterator = iterator.into_iter();
    let last = iterator.next();

    MergedRanges {
        values: iterator,
        last,
    }
}

impl<I> Iterator for MergedRanges<I>
where
    I: Iterator<Item = Range<i32>>,
{
    type Item = Range<i32>;

    fn next(&mut self) -> Option<Range<i32>> {
        // Are we still in the loop?
        if let Some(mut last) = self.last.clone() {
            for new in &mut self.values {
                if last.end < new.start {
                    self.last = Some(new);
                    return Some(last);
                }

                last.end = cmp::max(last.end, new.end);
            }

            self.last = None;
            return Some(last);
        }

        None
    }
}
