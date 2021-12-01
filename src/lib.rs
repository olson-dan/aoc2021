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
    input.windows(2).filter(|x| x[0] < x[1]).count()
}

#[aoc(day1, part2)]
fn day1_part2(input: &[usize]) -> usize {
    let input = input.windows(3).map(|x| x.iter().sum()).collect::<Vec<_>>();
    day1_part1(&input)
}

/*
#[aoc_generator(day2)]
fn day2_input(s: &str) -> Vec<usize> {
    s.trim()
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

#[aoc(day2, part1)]
fn day2_part1(input: &[usize]) -> usize {
    let mut answer = 0;
    for x in input.windows(2) {
        if x[1] > x[0] {
            answer += 1;
        }
    }
    answer
}

#[aoc(day2, part2)]
fn day2_part2(input: &[usize]) -> usize {
    let input = input.windows(3).map(|x| x.iter().sum()).collect::<Vec<_>>();
    day2_part1(&input)
}
*/

aoc_lib! { year = 2021 }
