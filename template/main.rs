use aoc;
// use intcode;
use std::iter::*;

fn part1(_: &Vec<i64>) -> i64 {
    0
}

fn part2(_: &Vec<i64>) -> i64 {
    0
}

fn parse(lines: &Vec<String>) -> Vec<i64> {
    lines.iter().map(|x| x.parse::<i64>().unwrap()).collect()
}

fn main() {
    let (part, lines) = aoc::read_lines();
    //let parsed = aoc::parse_intcode(&lines);
    let parsed = parse(&lines);
    let result = if part == 1 {
        part1(&parsed)
    } else {
        part2(&parsed)
    };
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    // use super::part1;

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(&vec![0]), 0);
    // }
}
