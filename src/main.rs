use std::{any::type_name, fmt::Display, time::Instant};

use aoc_2025::*;

fn time<F, N>(f: F, input: &str)
where
    F: FnOnce(&str) -> N,
    N: Display,
{
    let start = Instant::now();
    let answer = f(input);
    let elapsed = start.elapsed();

    println!(
        "{} Time {} us: {answer}",
        type_name::<F>().trim_start_matches("aoc2024::days::"),
        elapsed.as_micros()
    );
}

fn main() {
    let start = Instant::now();

    time(day1::a, day1::INPUT);
    time(day1::b, day1::INPUT);

    let elapsed = start.elapsed();

    println!("Total Time {} us", elapsed.as_micros());
}
