use aoc_runner_derive::{aoc, aoc_generator, aoc_lib};
use std::collections::HashMap;

#[aoc_generator(day1)]
fn day1_input(s: &str) -> Vec<usize> {
    s.trim()
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
fn day1_part1(input: &[usize]) -> usize {
    input.windows(2).filter(|x| x[1] > x[0]).count()
}

#[aoc(day1, part2)]
fn day1_part2(input: &[usize]) -> usize {
    input.windows(4).filter(|x| x[3] > x[0]).count()
}

#[aoc_generator(day2)]
fn day2_input(s: &str) -> Vec<(String, isize)> {
    s.trim()
        .lines()
        .map(|x| {
            let mut x = x.split(|c: char| c.is_whitespace());
            (
                x.next().unwrap().to_string(),
                x.next().unwrap().parse::<isize>().unwrap(),
            )
        })
        .collect()
}

#[aoc(day2, part1)]
fn day2_part1(input: &[(String, isize)]) -> isize {
    let mut h = 0;
    let mut d = 0;
    for x in input {
        match x.0.as_ref() {
            "forward" => h += x.1,
            "down" => d += x.1,
            "up" => d -= x.1,
            _ => unreachable!(),
        }
    }
    h * d
}

#[aoc(day2, part2)]
fn day2_part2(input: &[(String, isize)]) -> isize {
    let mut h = 0;
    let mut d = 0;
    let mut a = 0;
    for x in input {
        match x.0.as_ref() {
            "forward" => {
                h += x.1;
                d += a * x.1;
            }
            "down" => a += x.1,
            "up" => a -= x.1,
            _ => unreachable!(),
        }
    }
    h * d
}

aoc_lib! { year = 2021 }
