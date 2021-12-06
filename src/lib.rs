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

#[derive(Default, Clone)]
struct Board {
    values: Vec<Option<usize>>,
}

impl Board {
    fn won(&self) -> bool {
        let horiz = [0..5, 5..10, 10..15, 15..20, 20..25];
        // horiz
        for range in horiz {
            if self.values[range].iter().all(|x| matches!(x, None)) {
                return true;
            }
        }
        // vertical
        for x in 0..5 {
            if self
                .values
                .iter()
                .enumerate()
                .filter(|(i, _)| *i % 5 == x)
                .all(|(_, x)| matches!(x, None))
            {
                return true;
            }
        }
        false
    }

    fn kill(&mut self, number: usize) {
        for x in self.values.iter_mut() {
            if matches!(x, Some(n) if *n == number) {
                *x = None;
                return;
            }
        }
    }
}

#[derive(Clone)]
struct Bingo {
    boards: Vec<Board>,
    numbers: Vec<usize>,
}

#[aoc_generator(day4)]
fn day4_input(s: &str) -> Bingo {
    let mut lines = s.trim().lines();
    let numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let mut boards = Vec::new();
    while let Some(_) = lines.next() {
        let mut board = Board::default();
        for _ in 0..5 {
            board.values.extend(
                lines
                    .next()
                    .unwrap()
                    .trim()
                    .split_whitespace()
                    .map(|x| Some(x.parse::<usize>().unwrap())),
            );
        }
        boards.push(board);
    }
    Bingo { boards, numbers }
}

#[aoc(day4, part1)]
fn day4_part1(input: &Bingo) -> usize {
    let mut input = input.clone();
    for x in input.numbers.into_iter() {
        for board in input.boards.iter_mut() {
            board.kill(x);
            if board.won() {
                let sum: usize = board.values.iter().filter_map(|x| x.as_ref()).sum();
                return x * sum;
            }
        }
    }
    unreachable!();
}

#[aoc(day4, part2)]
fn day4_part2(input: &Bingo) -> usize {
    let mut input = input.clone();
    let mut last = false;
    for x in input.numbers.into_iter() {
        for board in input.boards.iter_mut() {
            board.kill(x);
            if last && board.won() {
                let sum: usize = board.values.iter().filter_map(|x| x.as_ref()).sum();
                return x * sum;
            }
        }
        input.boards.retain(|x| !x.won());
        if input.boards.len() == 1 {
            last = true;
        }
    }
    unreachable!();
}

#[aoc_generator(day5)]
fn day5_input(s: &str) -> Vec<(isize, isize, isize, isize)> {
    s.trim()
        .lines()
        .map(|x| {
            let mut it = x.split(" -> ");
            let mut a = it.next().unwrap().split(',');
            let mut b = it.next().unwrap().split(',');
            (
                a.next().unwrap().parse::<isize>().unwrap(),
                a.next().unwrap().parse::<isize>().unwrap(),
                b.next().unwrap().parse::<isize>().unwrap(),
                b.next().unwrap().parse::<isize>().unwrap(),
            )
        })
        .collect()
}

#[aoc(day5, part1)]
fn day5_part1(input: &[(isize, isize, isize, isize)]) -> usize {
    let mut map: HashMap<(isize, isize), isize> = HashMap::new();
    for inp in input.into_iter() {
        let (x1, y1, x2, y2) = *inp;
        if x1 == x2 {
            for y in std::cmp::min(y1, y2)..=std::cmp::max(y1, y2) {
                *map.entry((x1, y)).or_default() += 1;
            }
        } else if y1 == y2 {
            for x in std::cmp::min(x1, x2)..=std::cmp::max(x1, x2) {
                *map.entry((x, y1)).or_default() += 1;
            }
        }
    }
    map.iter().filter(|(_, v)| **v > 1).count()
}

#[aoc(day5, part2)]
fn day5_part2(input: &[(isize, isize, isize, isize)]) -> usize {
    let mut map: HashMap<(isize, isize), isize> = HashMap::new();
    for inp in input.into_iter() {
        let (x1, y1, x2, y2) = *inp;
        if x1 == x2 {
            for y in std::cmp::min(y1, y2)..=std::cmp::max(y1, y2) {
                *map.entry((x1, y)).or_default() += 1;
            }
        } else if y1 == y2 {
            for x in std::cmp::min(x1, x2)..=std::cmp::max(x1, x2) {
                *map.entry((x, y1)).or_default() += 1;
            }
        } else {
            let xstep = if x1 < x2 { 1 } else { -1 };
            let ystep = if y1 < y2 { 1 } else { -1 };
            let mut x = x1;
            let mut y = y1;
            loop {
                *map.entry((x, y)).or_default() += 1;
                if (x, y) == (x2, y2) {
                    break;
                }
                x += xstep;
                y += ystep;
            }
        }
    }
    map.iter().filter(|(_, v)| **v > 1).count()
}

#[aoc_generator(day6)]
fn day6_input(s: &str) -> Vec<usize> {
    let s = s.trim().lines().next().unwrap();
    s.split(',').map(|x| x.parse::<usize>().unwrap()).collect()
}

#[aoc(day6, part1)]
fn day6_part1(input: &[usize]) -> usize {
    let days = 80;
    let mut count = 0;
    let mut input = input.to_vec();
    for x in input.iter_mut() {
        count += 1; // self
        let mut spawn_nine = [0usize; 9];
        let mut spawn_seven = [0usize; 7];
        spawn_seven[*x] = 1;
        for _ in 0..days {
            spawn_seven[0] += spawn_nine[0];
            let spawned = spawn_seven[0];
            spawn_seven.rotate_left(1);
            spawn_nine.rotate_left(1);
            spawn_nine[8] = spawned;
            count += spawned;
        }
    }
    count
}

#[aoc(day6, part2)]
fn day6_part2(input: &[usize]) -> usize {
    let days = 256;
    let mut count = 0;
    let mut input = input.to_vec();
    for x in input.iter_mut() {
        count += 1; // self
        let mut spawn_nine = [0usize; 9];
        let mut spawn_seven = [0usize; 7];
        spawn_seven[*x] = 1;
        for _ in 0..days {
            spawn_seven[0] += spawn_nine[0];
            let spawned = spawn_seven[0];
            spawn_seven.rotate_left(1);
            spawn_nine.rotate_left(1);
            spawn_nine[8] = spawned;
            count += spawned;
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;
}

aoc_lib! { year = 2021 }
