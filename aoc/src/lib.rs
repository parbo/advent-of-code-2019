use num;
use pancurses;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::*;
use std::path::Path;

pub use num::integer::*;
pub use serde_scan::from_str;
pub use serde_scan::scan;

pub fn cum_sum<T: num::Num + Copy>(a: &[T]) -> Vec<T> {
    a.iter()
        .scan(T::zero(), |state, x| {
            *state = *state + *x;
            Some(*state)
        })
        .collect()
}

pub fn range_sum_inclusive<T: num::Num + Copy>(cum_sum: &[T], a: usize, b: usize) -> T {
    if b < a {
        T::zero()
    } else {
        if a == 0 {
            cum_sum[b]
        } else {
            cum_sum[b] - cum_sum[a - 1]
        }
    }
}

pub fn range_sum<T: num::Num + Copy>(cum_sum: &[T], a: usize, b: usize) -> T {
    if b > 0 {
        range_sum_inclusive(cum_sum, a, b - 1)
    } else {
        T::zero()
    }
}

pub struct GridValue {
    x: i128,
    y: i128,
    end_row: bool,
    value: Option<i128>,
}

pub struct SparseGrid<'a> {
    grid: &'a HashMap<(i128, i128), i128>,
    min_x: i128,
    max_x: i128,
    min_y: i128,
    max_y: i128,
    x: i128,
    y: i128,
}

impl<'a> SparseGrid<'a> {
    fn new(grid: &'a HashMap<(i128, i128), i128>) -> SparseGrid<'a> {
        let min_x = grid.iter().map(|p| (p.0).0).min().unwrap();
        let max_x = grid.iter().map(|p| (p.0).0).max().unwrap();
        let min_y = grid.iter().map(|p| (p.0).1).min().unwrap();
        let max_y = grid.iter().map(|p| (p.0).1).max().unwrap();
	SparseGrid { grid, min_x, max_x, min_y, max_y, x: min_x, y: min_y }
    }
}

impl<'a> Iterator for SparseGrid<'a> {
    type Item = GridValue;

    fn next(&mut self) -> Option<Self::Item> {
	let mut next_x = self.x + 1;
	let mut next_y = self.y;
	let mut end_row = false;
	if next_x > self.max_x {
	    next_x = self.min_x;
	    end_row = true;
	    next_y += 1;
	}
	if next_x > self.max_x || next_y > self.max_y {
	    None
	} else {
	    self.x = next_x;
	    self.y = next_y;
	    let v = if let Some(x) = self.grid.get(&(next_x, next_y)) {
		Some(*x)
	    } else {
		None
	    };
	    Some(GridValue{x: self.x - self.min_y, y: self.y - self.min_y, end_row: end_row, value: v})
	}
    }
}

// impl Grid for Vec<Vec<i128>> {
//     fn get_value(&self, pos: (i128, i128)) -> Option<i128> {
//         let (x, y) = pos;
//         if let Some(line) = self.get(y as usize) {
//             if let Some(c) = line.get(x as usize) {
//                 return Some(*c);
//             }
//         }
//         None
//     }
//     fn extents(&self) -> ((i128, i128), (i128, i128)) {
//         if self.len() > 0 {
//             if self[0].len() > 0 {
//                 return (
//                     (0, (self[0].len() - 1) as i128),
//                     (0, (self.len() - 1) as i128),
//                 );
//             }
//         }
//         ((0, 0), (0, 0))
//     }
// }

pub trait GridDrawer<G>
where
    G: Iterator<Item = GridValue>,
{
    fn draw(&mut self, area: &mut G);
}

pub struct NopGridDrawer {}

impl<G> GridDrawer<G> for NopGridDrawer
where
    G: Iterator<Item = GridValue>,
{
    fn draw(&mut self, _: &mut G) {}
}

pub struct PrintGridDrawer<F>
where
    F: Fn(i128) -> char,
{
    to_ch: F,
}

impl<F> PrintGridDrawer<F>
where
    F: Fn(i128) -> char,
{
    pub fn new(to_ch: F) -> PrintGridDrawer<F> {
        PrintGridDrawer { to_ch }
    }

    fn to_char(&self, col: i128) -> char {
        (self.to_ch)(col)
    }
}

impl<F, G> GridDrawer<G> for PrintGridDrawer<F>
where
    F: Fn(i128) -> char,
    G: Iterator<Item = GridValue>,
{
    fn draw(&mut self, area: &mut G) {
	loop {
	    match area.next() {
		Some(gv) => {
		    let ch = if let Some(v) = gv.value {
			self.to_char(v)
		    } else {
			' '
		    };
		    print!("{}", ch);
		    if gv.end_row {
			println!();
		    }
		},
		_ => break
	    }
        }
    }
}

pub struct CursesGridDrawer<F>
where
    F: Fn(i128) -> char,
{
    window: pancurses::Window,
    to_ch: F,
}

impl<F> CursesGridDrawer<F>
where
    F: Fn(i128) -> char,
{
    pub fn new(to_ch: F) -> CursesGridDrawer<F> {
        let window = pancurses::initscr();
        pancurses::nl();
        pancurses::noecho();
        pancurses::curs_set(0);
        window.keypad(true);
        window.scrollok(true);
        window.nodelay(true);
        CursesGridDrawer { window, to_ch }
    }

    fn to_char(&self, col: i128) -> char {
        (self.to_ch)(col)
    }
}

impl<F> Drop for CursesGridDrawer<F>
where
    F: Fn(i128) -> char,
{
    fn drop(&mut self) {
        pancurses::endwin();
    }
}

impl<F, G> GridDrawer<G> for CursesGridDrawer<F>
where
    F: Fn(i128) -> char,
    G: Iterator<Item = GridValue>,
{
    fn draw(&mut self, area: &mut G) {
        self.window.clear();
	loop {
	    match area.next() {
		Some(gv) => {
		    if let Some(v) = gv.value {
			self.window.mvaddch(gv.y as i32, gv.x as i32, self.to_char(v));
		    }
		}
		_ => break,
	    }
	}
	if let Some(pancurses::Input::Character(c)) = self.window.getch() {
            if c == 'q' {
                pancurses::endwin();
                std::process::exit(0);
            }
        }
        self.window.refresh();
    }
}

// TODO: improve and generalize
pub struct Tree {
    things: HashSet<String>,
    children: HashMap<String, Vec<String>>,
}

impl Tree {
    pub fn new() -> Tree {
        Tree {
            things: HashSet::new(),
            children: HashMap::new(),
        }
    }

    pub fn things(&self) -> impl Iterator<Item = &String> {
        self.things.iter()
    }

    pub fn insert(&mut self, parent: &str, child: &str) {
        self.things.insert(parent.to_string());
        self.things.insert(child.to_string());
        self.children
            .entry(parent.to_string())
            .or_insert(Vec::new())
            .push(child.to_string());
    }

    pub fn depth_from_to(&self, from: &str, to: &str) -> Option<i64> {
        self.depth_from_to_recursive(from, to, 0)
    }

    fn depth_from_to_recursive(&self, from: &str, to: &str, depth: i64) -> Option<i64> {
        if from == to {
            return Some(depth);
        }
        if let Some(v) = self.children.get(from) {
            for t in v {
                if let Some(x) = self.depth_from_to_recursive(t, to, depth + 1) {
                    return Some(x);
                }
            }
        }
        return None;
    }
}

pub fn read_lines() -> (i32, Vec<String>) {
    let args: Vec<String> = env::args().collect();
    let part = args[1].parse::<i32>().unwrap();
    let filename = &args[2];

    let input = File::open(Path::new(filename)).unwrap();
    let buffered = BufReader::new(input);
    (
        part,
        buffered
            .lines()
            .filter_map(Result::ok)
            .map(|x| x.trim().to_string())
            .collect(),
    )
}

pub fn parse_intcode(lines: &Vec<String>) -> Vec<i128> {
    let result: Vec<i128> = lines[0]
        .split(|c| c == ',')
        .map(|s| s.trim())
        .map(|v| v.parse::<i128>().unwrap())
        .collect();
    result
}
