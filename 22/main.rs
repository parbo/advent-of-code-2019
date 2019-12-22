use aoc;
use std::iter::*;
use aoc::BigInt;

enum Shuffle {
    DealIntoNewStack,
    Cut(i64),
    DealWithIncrement(usize),
}

fn pos_mod(x: &BigInt, y: &BigInt) -> BigInt {
    ((a % b) + b) % b
}

fn shuffle_idx(how: &Vec<Shuffle>, len: &BigInt, idx: &BigInt) -> BigInt {
    let mut new_idx = idx.clone();
    // println!("====== {}", idx);
    for s in how {
        match s {
            Shuffle::DealIntoNewStack => {
		new_idx = len - new_idx - 1;
            }
            Shuffle::Cut(x) => {
		let xx : BigInt = aoc::FromPrimitive::from_i64(*x).unwrap();
                new_idx = new_idx.clone() + len - xx;
            }
            Shuffle::DealWithIncrement(x) => {
		let xx : BigInt = aoc::FromPrimitive::from_usize(*x).unwrap();
                new_idx = new_idx.clone() * xx;
            }
        }
	// println!("{}, {}", new_idx, pos_mod(new_idx, len));
    }
    new_idx
}

fn shuffle(how: &Vec<Shuffle>, len: i128) -> Vec<BigInt> {
    let deck: Vec<BigInt> = (0..len).into_iter().map(|x| aoc::FromPrimitive::from_i128(x).unwrap()).collect();
    let mut new_deck : Vec<BigInt> = Vec::new();
    new_deck.resize(len as usize, aoc::FromPrimitive::from_i32(0).unwrap());
    let big_len : BigInt = aoc::FromPrimitive::from_i128(len).unwrap();
    for i in 0..len {
	let ii = aoc::FromPrimitive::from_i128(i).unwrap();
        let x = shuffle_idx(how, &big_len, &ii);
	println!("x: {}, i: {}", x, i);
	let ii : usize = aoc::ToPrimitive::to_usize(&i).unwrap();
	let xx : usize = aoc::ToPrimitive::to_usize(&pos_mod(&x, &big_len)).unwrap();
        new_deck[xx] = deck[ii].clone();
    }
    new_deck
}

fn part1(input: &Vec<Shuffle>) -> BigInt {
    let len = 10007i128;
    let shuffled = shuffle(input, len);
    aoc::FromPrimitive::from_usize(shuffled
        .iter()
        .enumerate()
        .find(|(_, x)| **x == aoc::FromPrimitive::from_i32(2019).unwrap())
        .unwrap()
        .0).unwrap()
}

fn part2(input: &Vec<Shuffle>) -> BigInt {
    let len : BigInt = aoc::FromPrimitive::from_i128(119315717514047i128).unwrap();
    let times : BigInt = aoc::FromPrimitive::from_i128(101741582076661i128).unwrap();
    // println!(
    //     "{}, {}, {}, {}",
    //     len / times,
    //     len % times,
    //     aoc::gcd(len, times),
    //     aoc::lcm(len, times)
    // );
    let mut i = 0;
    let wanted : BigInt = aoc::FromPrimitive::from_i32(2020).unwrap();
    let mut ix = wanted.clone();
    let new_ix = pos_mod(&shuffle_idx(input, &len, &ix), &len);
    let nix : i128 = aoc::ToPrimitive::to_i128(&new_ix).unwrap();
    // let ans = (new_ix.clone() - ix.clone());
    // let ans2 = pos_mod(&pos_mod(&ans, &len), &times);
    println!("nix: {}", nix);
    new_ix
}

fn parse(lines: &Vec<String>) -> Vec<Shuffle> {
    let mut res = vec![];
    for line in lines {
        if line == "deal into new stack" {
            res.push(Shuffle::DealIntoNewStack);
        } else if let Ok(x) = aoc::scan!("cut {}" <- line) {
            res.push(Shuffle::Cut(x));
        } else if let Ok(x) = aoc::scan!("deal with increment {}" <- line) {
            res.push(Shuffle::DealWithIncrement(x));
        } else {
            panic!();
        }
    }
    res
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
    use aoc::BigInt;
    use super::{parse, shuffle};

    fn bigv(v: &[i32]) -> Vec<BigInt> {
    	let out : Vec<BigInt> = v.iter().map(|x| aoc::FromPrimitive::from_i32(*x).unwrap()).collect();
	out
    }

    #[test]
    fn test_shuffle_rev() {
        let input = vec![
            "deal into new stack".to_string(),
        ];
        let how = parse(&input);
        assert_eq!(shuffle(&how, 10), bigv(&vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]));
    }

    #[test]
    fn test_shuffle_rev2() {
        let input = vec![
            "deal into new stack".to_string(),
            "deal into new stack".to_string(),
        ];
        let how = parse(&input);
        assert_eq!(shuffle(&how, 10), bigv(&vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));
    }

    #[test]
    fn test_shuffle_incr() {
        let input = vec![
            "deal with increment 1".to_string(),
        ];
        let how = parse(&input);
        assert_eq!(shuffle(&how, 10), bigv(&vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));
    }

    #[test]
    fn test_shuffle_inc3() {
        let input = vec![
            "deal with increment 3".to_string(),
        ];
        let how = parse(&input);
        assert_eq!(shuffle(&how, 10), bigv(&vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3]));
    }

    #[test]
    fn test_shuffle_cut3() {
        let input = vec![
            "cut 3".to_string(),
        ];
        let how = parse(&input);
        assert_eq!(shuffle(&how, 10), bigv(&vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2]));
    }

    #[test]
    fn test_shuffle_cutminus4() {
        let input = vec![
            "cut -4".to_string(),
        ];
        let how = parse(&input);
        assert_eq!(shuffle(&how, 10), bigv(&vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_shuffle_inc_rev() {
        let input = vec![
            "deal with increment 3".to_string(),
            "deal into new stack".to_string(),
        ];
        let how = parse(&input);
        assert_eq!(shuffle(&how, 10), bigv(&vec![3, 6, 9, 2, 5, 8, 1, 4, 7, 0]));
    }

    #[test]
    fn test_shuffle_1() {
        let input = vec![
            "deal with increment 7".to_string(),
            "deal into new stack".to_string(),
            "deal into new stack".to_string(),
        ];
        let how = parse(&input);
        assert_eq!(shuffle(&how, 10), bigv(&vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]));
    }

    #[test]
    fn test_shuffle_2() {
        let input = vec![
            "cut 6".to_string(),
            "deal with increment 7".to_string(),
            "deal into new stack".to_string(),
        ];
        let how = parse(&input);
        assert_eq!(shuffle(&how, 10), bigv(&vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6]));
    }

    #[test]
    fn test_shuffle_3() {
        let input = vec![
            "deal with increment 7".to_string(),
            "deal with increment 9".to_string(),
            "cut -2".to_string(),
        ];
        let how = parse(&input);
        assert_eq!(shuffle(&how, 10), bigv(&vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9]));
    }

    #[test]
    fn test_shuffle_4() {
        let input = vec![
            "deal into new stack".to_string(),
            "cut -2".to_string(),
            "deal with increment 7".to_string(),
            "cut 8".to_string(),
            "cut -4".to_string(),
            "deal with increment 7".to_string(),
            "cut 3".to_string(),
            "deal with increment 9".to_string(),
            "deal with increment 3".to_string(),
            "cut -1".to_string(),
        ];
        let how = parse(&input);
        assert_eq!(shuffle(&how, 10), bigv(&vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6]));
    }
}
