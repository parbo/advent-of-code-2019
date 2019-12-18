use aoc;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::iter::*;
use pathfinding::prelude::dijkstra;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct KeyState {
    state: u32,
}

impl KeyState {
    fn set(&mut self, ch: char) {
        let bit = (ch as u32) - 97;
        self.state |= 1 << bit;
    }

    fn get(&self, ch: char) -> bool {
        let bit = (ch as u32) - 97;
        let mask = 1 << bit;
        (self.state & mask) == mask
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Map<'a> {
    map: &'a Vec<Vec<char>>,
    position: (usize, usize),
    key_state: KeyState,
}

impl<'a> Map<'a> {
    fn new(map: &'a Vec<Vec<char>>, position: (usize, usize), key_state: KeyState) -> Map<'a> {
        Map {
            map,
	    position,
            key_state,
        }
    }
}

impl<'a> Map<'a> {
    fn successors(&self) -> Vec<(Map<'a>, usize)> {
	let mut n = vec![];
	let pos = self.position;
	let y = pos.0 as i64;
	let x = pos.1 as i64;
	let w = self.map[0].len() as i64;
	let h = self.map.len() as i64;
	for (ny, nx) in &[(y - 1, x), (y, x - 1), (y, x + 1), (y + 1, x)] {
            if *nx > w || *ny > h || *ny < 0 || *nx < 0 {
		continue;
            }
            let p = (*ny as usize, *nx as usize);
            let ch = self.map[p.0][p.1];
            if ch == '#' {
		// No action
            } else if ch == '.' || ch.is_ascii_lowercase() || (ch.is_ascii_uppercase() && self.key_state.get(ch.to_ascii_lowercase())) {
		n.push((Map::new(self.map, (*ny as usize, *nx as usize), self.key_state), 1));
            }
	}
	n
    }
}

fn find_path<'a>(map: &'a Vec<Vec<char>>, keys: KeyState, start: (usize, usize), goal: (usize, usize)) -> Option<(Vec<Map<'a>>, usize)> {
    let m = Map::new(map, start, keys);
    dijkstra(&m, |p| p.successors(), |p| p.position == goal)
}

fn find_keys(map: &Vec<Vec<char>>) -> HashMap<(usize, usize), char> {
    let mut things = HashMap::new();
    let h = map.len();
    let w = map[0].len();
    for y in 0..h {
        for x in 0..w {
            let ch = map[y][x];
            if ch.is_ascii_alphabetic() && ch.is_ascii_lowercase() {
                things.insert((y, x), ch);
            }
        }
    }
    things
}

fn find_self(map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    let h = map.len();
    let w = map[0].len();
    for y in 0..h {
        for x in 0..w {
            if map[y][x] == '@' {
                return Some((y, x));
            }
        }
    }
    None
}

fn total_cost<'a>(paths: &Vec<(Vec<Map<'a>>, usize)>) -> usize {
    paths.iter().map(|x| x.0.len()).sum()
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct MapState {
    positions: Vec<(usize, usize)>,
    keys: KeyState,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct PathState<'a> {
    cost: usize,
    map_state: MapState,
    paths: Vec<(Vec<Map<'a>>, usize)>,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl<'a> Ord for PathState<'a> {
    fn cmp(&self, other: &PathState) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.map_state.positions.cmp(&self.map_state.positions))
    }
}

// `PartialOrd` needs to be implemented as well.
impl<'a> PartialOrd for PathState<'a> {
    fn partial_cmp(&self, other: &PathState) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve<'a>(map: &'a Vec<Vec<char>>, curr: &Vec<(usize, usize)>) -> usize {
    let mut dist = HashMap::new();
    let mut frontier: BinaryHeap<PathState> = BinaryHeap::new();
    let all_keys = find_keys(&map);
    frontier.push(PathState {
        cost: 0,
        map_state: MapState {
            positions: curr.clone(),
            keys: KeyState { state: 0 },
        },
        paths: vec![],
    });

    let mut goal_cost = None;
    let mut res = vec![];
    let mut last_cost = 0;
    let mut cached = 0;
    let mut total = 0;
    while let Some(PathState {
        cost,
        map_state,
        paths,
    }) = frontier.pop()
    {
        let keys: Vec<_> = all_keys
            .iter()
            .filter(|(_, v)| !map_state.keys.get(**v))
            .collect();
        if cost / 100 != last_cost {
            println!(
                "keys: {:?}, cost: {}, cache: {}%, total: {}",
                keys.len(),
                cost,
                100 * cached / total,
                total
            );
            last_cost = cost / 100;
        }
        if keys.len() == 0 {
            if let Some(gc) = goal_cost {
                if cost == gc {
                    res.push((map_state.clone(), paths.clone()));
                }
            } else {
                goal_cost = Some(cost);
                res.push((map_state.clone(), paths.clone()));
            }
        }

        if let Some(gc) = goal_cost {
            if cost > gc {
                break;
            }
        }

        // Important as we may have already found a better way
        if let Some(x) = dist.get(&map_state) {
            if cost > *x {
                continue;
            }
        }

        // println!("at {:?}, {}, looking for {:?} keys", map_state.position, cost, keys.len());
        for (pos, key) in keys {
            let pos_len = map_state.positions.len();
            for i in 0..pos_len {
                let rob_pos = map_state.positions[i];
                total += 1;
                if let Some(p) = find_path(&map, map_state.keys, rob_pos, *pos) {
                    println!("found path from {:?} {}, to {} at {:?}", map_state.positions, i, key, pos);
                    // println!("{:?}", p);
                    let mut new_paths = paths.clone();
                    new_paths.push(p.clone());

                    let mut new_keys = map_state.keys;
                    new_keys.set(*key);
                    p.0.iter().for_each(|p| {
                        if let Some(k) = all_keys.get(&p.position) {
                            new_keys.set(*k);
                        }
                    });
                    let mut new_pos = map_state.positions.clone();
                    new_pos[i] = *pos;
                    let next = PathState {
                        cost: total_cost(&new_paths),
                        map_state: MapState {
                            positions: new_pos,
                            keys: new_keys,
                        },
                        paths: new_paths,
                    };

                    let d = if let Some(x) = dist.get(&next.map_state) {
                        *x
                    } else {
                        std::usize::MAX
                    };

                    // println!("next: {}, d: {}", next.cost, d);

                    // If so, add it to the frontier and continue
                    if next.cost < d {
                        // Relaxation, we have now found a better way
                        dist.insert(next.map_state.clone(), next.cost);
                        frontier.push(next);
                    }
                }
            }
        }
    }
    goal_cost.unwrap()
}

fn part1(map: &Vec<Vec<char>>) -> usize {
    let curr = find_self(&map).unwrap();
    let cv = vec![curr];
    solve(&map, &cv)
}

fn part2(map: &Vec<Vec<char>>) -> usize {
    let curr = find_self(&map).unwrap();
    let mut m = map.clone();
    m[curr.0][curr.1] = '#';
    m[curr.0][curr.1 + 1] = '#';
    m[curr.0][curr.1 - 1] = '#';
    m[curr.0 - 1][curr.1] = '#';
    m[curr.0 + 1][curr.1] = '#';
    m[curr.0 + 1][curr.1 + 1] = '.';
    m[curr.0 - 1][curr.1 + 1] = '.';
    m[curr.0 + 1][curr.1 - 1] = '.';
    m[curr.0 - 1][curr.1 - 1] = '.';

    let cv = vec![
        (curr.0 + 1, curr.1 + 1),
        (curr.0 - 1, curr.1 + 1),
        (curr.0 + 1, curr.1 - 1),
        (curr.0 - 1, curr.1 - 1),
    ];
    solve(&m, &cv)
}

fn parse(lines: &Vec<String>) -> Vec<Vec<char>> {
    lines.iter().map(|x| x.chars().collect()).collect()
}

fn main() {
    let (part, lines) = aoc::read_lines();
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
