use aoc;
use aoc::GridDrawer;
use std::collections::HashMap;
use std::iter::*;

fn make_grid(
    program: &Vec<i128>,
    x: i128,
    y: i128,
    w: i128,
    h: i128,
) -> HashMap<(i128, i128), i128> {
    let mut grid = HashMap::new();
    for yy in y..(y + h) {
        for xx in x..(x + w) {
            let mut m = intcode::Machine::new(program);
            m.add_input(xx as i128);
            m.add_input(yy as i128);
            if let Some(v) = m.run_to_next_output() {
                grid.insert((xx, yy), v);
            }
        }
    }
    grid
}

fn draw(program: &Vec<i128>, x: i128, y: i128, sq: i128, pad: i128) {
    let mut grid = make_grid(program, x - pad, y - pad, sq + 2 * pad, sq + 2 * pad);
    for yy in y..(y + sq) {
        for xx in x..(x + sq) {
            *grid.entry((xx, yy)).or_insert(0) += 10;
        }
    }
    let mut d = aoc::PrintGridDrawer::new(|ch| match ch {
        1 => '#',
        0 => '.',
        11 => 'o',
        _ => '!',
    });
    d.draw(&grid);
}

fn part1(program: &Vec<i128>) -> i128 {
    let grid = make_grid(program, 0, 0, 50, 50);
    grid.iter().filter(|(_, v)| **v == 1).count() as i128
}

fn get_beam_at(program: &Vec<i128>, start_x: i128, y: i128) -> (i128, i128) {
    let mut s = 0;
    let mut x = start_x;
    loop {
        let mut mach = intcode::Machine::new(program);
        mach.add_input(x as i128);
        mach.add_input(y as i128);
        if let Some(v) = mach.run_to_next_output() {
            if v == 1 && s == 0 {
                s = x;
            }
            if v == 0 && s > 0 {
                return (s, x);
            }
        }
        x += 1;
    }
}

fn part2(program: &Vec<i128>) -> i128 {
    let mut a = 0;
    let mut b = 10000;
    let sq = 100;
    let mut start_x = 0;
    let res = loop {
        let m = (a + b) / 2;
        let (s1, e1) = get_beam_at(program, start_x, m);
        let (s2, _e2) = get_beam_at(program, start_x, m + sq - 1);
        let s = e1 - sq;
        println!("y: {}, s: {}, s2: {}, {}", m, s, s2, e1 - s2);
        if s < s2 {
            a = m + 1;
	    start_x = s1 - 1;
        }
        if s > s2 {
            b = m - 1;
        }
        if s == s2 {
            break m;
        }
    };
    // Loop back to find the smallest
    let mut y = res;
    let mut fails = 0;
    let mut last_good = (0, 0);
    start_x -= 20;
    loop {
        let (_s1, e1) = get_beam_at(program, start_x, y);
        let (s2, _e2) = get_beam_at(program, start_x, y + sq - 1);
        let s = e1 - sq;
        if s2 == s {
            println!("backing up y: {}, s: {}, s2: {}, {}", y, s, s2, e1 - s2);
            last_good = (s, y);
            y -= 1;
        } else {
            println!("no can do y: {}, s: {}, s2: {}, {}", y, s, s2, e1 - s2);
            fails += 1;
            y -= 1;
            if fails > 20 {
                break;
            }
        }
    }
    let (x, y) = last_good;
    println!("x: {}, y: {}", x, y);
    draw(program, x, y, sq, 5);
    x * 10000 + y
}

fn main() {
    let (part, lines) = aoc::read_lines();
    let parsed = aoc::parse_intcode(&lines);
    let result = if part == 1 {
        part1(&parsed)
    } else {
        part2(&parsed)
    };
    println!("{}", result);
}
