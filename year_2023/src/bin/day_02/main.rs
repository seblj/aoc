use std::str::FromStr;

#[derive(Debug)]
struct Game {
    id: usize,
    cubes: Vec<Cubes>,
}

#[derive(Debug)]
struct Cubes {
    blue: usize,
    red: usize,
    green: usize,
}

impl Cubes {
    fn new() -> Self {
        Cubes {
            blue: 0,
            red: 0,
            green: 0,
        }
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game, rest) = s.split_once(':').unwrap();
        let cubes = rest
            .split(';')
            .map(|c| {
                c.split(',').fold(Cubes::new(), |mut acc, x| {
                    let (value, color) = x
                        .trim()
                        .split_once(' ')
                        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.trim()))
                        .unwrap();

                    match color {
                        "red" => acc.red = value,
                        "blue" => acc.blue = value,
                        "green" => acc.green = value,
                        _ => unreachable!(),
                    };
                    acc
                })
            })
            .collect::<Vec<Cubes>>();

        let id = game.split_once(' ').unwrap().1.parse::<usize>().unwrap();

        Ok(Game { id, cubes })
    }
}

fn task_one(input: &[String]) -> usize {
    input.iter().fold(0, |acc, s| {
        let game = Game::from_str(s).unwrap();
        match game
            .cubes
            .iter()
            .find(|c| c.red > 12 || c.green > 13 || c.blue > 14)
        {
            Some(_) => acc,
            None => acc + game.id,
        }
    })
}

fn task_two(input: &[String]) -> usize {
    input.iter().fold(0, |acc, s| {
        let game = Game::from_str(s).unwrap();
        let cubes = game
            .cubes
            .into_iter()
            .reduce(|mut c_acc, c| {
                c_acc.green = c.green.max(c_acc.green);
                c_acc.red = c.red.max(c_acc.red);
                c_acc.blue = c.blue.max(c_acc.blue);

                c_acc
            })
            .unwrap();

        acc + (cubes.red * cubes.blue * cubes.green)
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
