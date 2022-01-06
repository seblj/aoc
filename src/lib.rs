// Honorable mention to @sivertjoe for these great generic functions
// https://github.com/sivertjoe/Advent-of-Code-2021

pub fn read_input<T, P>(path: P) -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
    P: AsRef<std::path::Path>,
{
    use std::io::BufRead;
    let file = std::fs::File::open(path).unwrap();
    std::io::BufReader::new(file)
        .lines()
        .flatten()
        .map(|line| line.parse::<T>().unwrap())
        .collect()
}

pub fn time<F, T, U>(pre: &'static str, f: F, arg: T)
where
    F: Fn(T) -> U,
    U: std::fmt::Display,
{
    let t0 = std::time::Instant::now();
    let res = f(arg);
    let t1 = std::time::Instant::now();
    println!(
        "Task {}: {}\t({:.4}ms)",
        pre,
        res,
        t1.duration_since(t0).as_nanos() as f64 / 1_000_000.0
    );
}
