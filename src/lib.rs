use aoc_runner_derive::{aoc, aoc_generator, aoc_lib};
//use std::collections::HashMap;

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
            let mut x = x.split_whitespace();
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

#[aoc_generator(day3)]
fn day3_input(s: &str) -> Vec<String> {
    s.trim().lines().map(|x| x.to_string()).collect()
}

#[aoc(day3, part1)]
fn day3_part1(input: &[String]) -> usize {
    let mut gamma_str = String::new();
    let mut epsilon_str = String::new();
    let len = input[9].len();
    for i in 0..len {
        let mut ones = 0;
        let mut zeroes = 0;
        for x in input {
            match x.chars().skip(i).next() {
                Some('0') => zeroes += 1,
                Some('1') => ones += 1,
                _ => unreachable!(),
            }
        }
        if ones > zeroes {
            gamma_str.push('1');
            epsilon_str.push('0');
        } else {
            gamma_str.push('0');
            epsilon_str.push('1');
        }
    }
    usize::from_str_radix(&gamma_str, 2).unwrap() * usize::from_str_radix(&epsilon_str, 2).unwrap()
}

#[aoc(day3, part2)]
fn day3_part2(input: &[String]) -> usize {
    let input = input
        .iter()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut i = 0;
    let mut ox = input.clone();
    let mut co2 = input.clone();
    while co2.len() != 1 || ox.len() != 1 {
        if ox.len() != 1 {
            let zeroes = ox
                .iter()
                .cloned()
                .filter(|x| x[i] == '0')
                .collect::<Vec<_>>();
            let ones = ox
                .iter()
                .cloned()
                .filter(|x| x[i] == '1')
                .collect::<Vec<_>>();
            if zeroes.len() <= ones.len() {
                ox = ones;
            } else {
                ox = zeroes;
            }
        }
        if co2.len() != 1 {
            let zeroes = co2
                .iter()
                .cloned()
                .filter(|x| x[i] == '0')
                .collect::<Vec<_>>();
            let ones = co2
                .iter()
                .cloned()
                .filter(|x| x[i] == '1')
                .collect::<Vec<_>>();
            if zeroes.len() > ones.len() {
                co2 = ones;
            } else {
                co2 = zeroes;
            }
        }
        i += 1;
    }
    let ox = ox[0]
        .iter()
        .rev()
        .fold((0, 0), |x, y| {
            if *y == '0' {
                (x.0, x.1 + 1)
            } else {
                (x.0 + (1usize << x.1), x.1 + 1)
            }
        })
        .0;
    let co2 = co2[0]
        .iter()
        .rev()
        .fold((0, 0), |x, y| {
            if *y == '0' {
                (x.0, x.1 + 1)
            } else {
                (x.0 + (1usize << x.1), x.1 + 1)
            }
        })
        .0;
    ox * co2
}

aoc_lib! { year = 2021 }
