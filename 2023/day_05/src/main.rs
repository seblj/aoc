use std::thread;

#[derive(Clone, Debug)]
struct MapLine {
    source_range_start: usize,
    dest_range_start: usize,
    range_length: usize,
}

#[derive(Clone, Debug)]
struct Map {
    map: Vec<MapLine>,
}

impl Map {
    fn find(&self, val: usize) -> usize {
        self.map
            .iter()
            .find_map(|m| {
                if (m.source_range_start..m.source_range_start + m.range_length).contains(&val) {
                    Some((val - m.source_range_start) + m.dest_range_start)
                } else {
                    None
                }
            })
            .unwrap_or(val)
    }
}

#[derive(Debug, Clone)]
struct Almanack {
    seeds: Vec<usize>,
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temp: Map,
    temp_to_humidity: Map,
    humidity_to_loc: Map,
}

fn parse(input: &[String]) -> Almanack {
    let seeds: Vec<usize> = input[0]
        .split_whitespace()
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();

    let mut v = Vec::from([
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ]);
    let mut idx = 0;
    for s in input.iter() {
        match s.as_str() {
            "seed-to-soil map:" => idx = 0,
            "soil-to-fertilizer map:" => idx = 1,
            "fertilizer-to-water map:" => idx = 2,
            "water-to-light map:" => idx = 3,
            "light-to-temperature map:" => idx = 4,
            "temperature-to-humidity map:" => idx = 5,
            "humidity-to-location map:" => idx = 6,
            _ => {}
        }

        if !s.chars().next().unwrap_or('_').is_ascii_digit() {
            continue;
        }

        let values: Vec<usize> = s
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        v[idx].push(MapLine {
            dest_range_start: values[0],
            source_range_start: values[1],
            range_length: values[2],
        })
    }

    Almanack {
        seeds,
        seed_to_soil: Map { map: v[0].clone() },
        soil_to_fertilizer: Map { map: v[1].clone() },
        fertilizer_to_water: Map { map: v[2].clone() },
        water_to_light: Map { map: v[3].clone() },
        light_to_temp: Map { map: v[4].clone() },
        temp_to_humidity: Map { map: v[5].clone() },
        humidity_to_loc: Map { map: v[6].clone() },
    }
}

fn task_one(input: &[String]) -> usize {
    let almanack = parse(input);
    let mut locations = vec![];
    for seed in almanack.seeds.iter() {
        let soil = almanack.seed_to_soil.find(*seed);
        let fertilizer = almanack.soil_to_fertilizer.find(soil);
        let water = almanack.fertilizer_to_water.find(fertilizer);
        let light = almanack.water_to_light.find(water);
        let temp = almanack.light_to_temp.find(light);
        let humidity = almanack.temp_to_humidity.find(temp);
        let loc = almanack.humidity_to_loc.find(humidity);
        locations.push(loc);
    }
    *locations.iter().min().unwrap()
}

fn task_two(input: &[String]) -> usize {
    let almanack = parse(input);
    let seeds: Vec<(usize, usize)> = almanack
        .seeds
        .windows(2)
        .step_by(2)
        .map(|x| (x[0], x[1]))
        .collect();

    let mut all_locations = vec![];

    for _seed in seeds.into_iter() {
        for seed in _seed.0.._seed.0 + _seed.1 {
            let a = almanack.clone();

            let handle = thread::spawn(move || {
                let soil = a.seed_to_soil.find(seed);
                let fertilizer = a.soil_to_fertilizer.find(soil);
                let water = a.fertilizer_to_water.find(fertilizer);
                let light = a.water_to_light.find(water);
                let temp = a.light_to_temp.find(light);
                let humidity = a.temp_to_humidity.find(temp);
                a.humidity_to_loc.find(humidity)
            });

            let res = handle.join().unwrap();
            all_locations.push(res);
        }
    }

    *all_locations.iter().min().unwrap()
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
