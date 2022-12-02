use std::str::FromStr;

enum Shape {
    Rock,
    Paper,
    Scissor,
}

enum Outcome {
    Loss,
    Draw,
    Victory,
}

struct RockPaperScissor {
    me: Shape,
    opponent: Shape,
    fixed_result: Outcome,
}

impl FromStr for Shape {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissor,
            _ => unreachable!(),
        })
    }
}

impl FromStr for Outcome {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Victory,
            _ => unreachable!(),
        })
    }
}

impl FromStr for RockPaperScissor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (opponent, me) = s.split_once(' ').unwrap();
        let fixed_result = Outcome::from_str(me).unwrap();
        let opponent = Shape::from_str(opponent).unwrap();
        let me = Shape::from_str(me).unwrap();
        Ok(Self {
            opponent,
            me,
            fixed_result,
        })
    }
}

impl RockPaperScissor {
    fn game(input: &str) -> Self {
        Self::from_str(input).unwrap()
    }

    fn outcome(&self) -> Outcome {
        match self.opponent {
            Shape::Rock => match self.me {
                Shape::Rock => Outcome::Draw,
                Shape::Paper => Outcome::Victory,
                Shape::Scissor => Outcome::Loss,
            },
            Shape::Paper => match self.me {
                Shape::Rock => Outcome::Loss,
                Shape::Paper => Outcome::Draw,
                Shape::Scissor => Outcome::Victory,
            },
            Shape::Scissor => match self.me {
                Shape::Rock => Outcome::Victory,
                Shape::Paper => Outcome::Loss,
                Shape::Scissor => Outcome::Draw,
            },
        }
    }

    fn pick_shape(&self, fixed_result: &Outcome) -> Shape {
        match fixed_result {
            Outcome::Loss => match self.opponent {
                Shape::Rock => Shape::Scissor,
                Shape::Paper => Shape::Rock,
                Shape::Scissor => Shape::Paper,
            },
            Outcome::Draw => match self.opponent {
                Shape::Rock => Shape::Rock,
                Shape::Paper => Shape::Paper,
                Shape::Scissor => Shape::Scissor,
            },
            Outcome::Victory => match self.opponent {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissor,
                Shape::Scissor => Shape::Rock,
            },
        }
    }
}

trait Score {
    fn score(&self) -> i32;
}

impl Score for Shape {
    fn score(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissor => 3,
        }
    }
}

impl Score for Outcome {
    fn score(&self) -> i32 {
        match self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Victory => 6,
        }
    }
}

fn task_one(input: &[String]) -> i32 {
    input.iter().fold(0, |acc, input| {
        let game = RockPaperScissor::game(input);
        game.me.score() + game.outcome().score() + acc
    })
}

fn task_two(input: &[String]) -> i32 {
    input.iter().fold(0, |acc, input| {
        let game = RockPaperScissor::game(input);
        game.fixed_result.score() + game.pick_shape(&game.fixed_result).score() + acc
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
