use std::{fs, fmt};
// use std::collections::VecDeque;

const SHAPES: [&'static [u8]; 5] = [
    &[
        0b0011110
    ],
    &[
        0b0001000,
        0b0011100,
        0b0001000,
    ],
   &[
        0b0000100,
        0b0000100,
        0b0011100,
    ],
   &[
        0b0010000,
        0b0010000,
        0b0010000,
        0b0010000,
    ],
   &[
        0b0011000,
        0b0011000,
    ],
];

type Rock = Vec<u8>;

#[derive(Debug)]
struct Chamber {
    layers: Vec<u8>,
    // height: usize,
    // acc_height: usize, 
}

impl fmt::Display for Chamber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, l) in self.layers.iter().enumerate().rev() {
            writeln!(f, "{:07b} {}", l, i)?;
        }
        write!(f, "")
    }
}

impl Chamber {
    fn new() -> Self {
        Chamber {
            layers: Vec::new(),
        }
    }

    fn add(&mut self, at: usize, r: Rock) {
        let new: Vec<u8> = r.iter().rev().zip(self.layers[at..at+r.len()].iter()).map(|(a, b)| a | b).collect();
        self.layers.splice(at..at+r.len(), new.into_iter());
    }

    fn fill_space(&mut self) {
        let s = 4;
        let spaces = s - self.layers.iter().rev()
            .take(s)
            .take_while(|l| **l == 0 )
            .count();
        for _ in 0..spaces {
            self.layers.push(0);
        }
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Down,
}


fn main() {
    let input = fs::read_to_string("day17/input.txt").unwrap();
    println!("{}", solve(&input, 2022));
    println!("{}", solve(&input, 1000000000000));
}

fn try_move(dir: &Direction, r: &mut Rock, c: &mut Chamber, offset: usize) -> bool {
    match dir {
        Direction::Down => {
            if c.layers.len() - offset == 0 {
                c.add(0, r.clone());
                return true
            }
            for (e, l) in r.iter().rev().zip(c.layers[c.layers.len()-offset-1..].iter()) {
                if e & l > 0 {
                    c.add(c.layers.len()-offset, r.clone());
                    return true
                }
            }
        }
        Direction::Left => {
            let mut tr = r.clone();
            for e in tr.iter_mut() {
                *e = e.rotate_left(1);
            }
            for e in tr.iter() {
                if e.leading_ones() > 0 {
                    return false
                }
            }
            for (e, l) in tr.iter().rev().zip(c.layers[c.layers.len()-offset..].iter()) {
                if e & l > 0 {
                    return false
                }
            }
            *r = tr;
        }
        Direction::Right => {
            let mut tr = r.clone();
            for e in tr.iter_mut() {
                *e = e.rotate_right(1);
            }
            for e in tr.iter() {
                if e.leading_ones() > 0 {
                    return false
                }
            }
            for (e, l) in tr.iter().rev().zip(c.layers[c.layers.len()-offset..].iter()) {
                if e & l > 0 {
                    return false
                }
            }
            *r = tr;
        }
    }
    false
}

fn solve(input: &str, rounds: usize) -> usize {
    let jets: Vec<Direction> = input
        .trim()
        .chars()
        .map(|c| match c {
            '>' => Direction::Right,
            '<' => Direction::Left,
            _ => unimplemented!("Unknown jet char"),
        })
        .collect();

    let mut c = Chamber::new();
    let mut jet_i = 0;
    let mut shape_i = 0;
    for i in 0..rounds {
        if i % 100000 == 0 {
            println!("round {i}");
        }
        c.fill_space();
        let mut rock: Rock = Vec::from(SHAPES[shape_i]);
        let mut rock_stopped = false;
        let mut offset = 1;
        let mut down = false;
        while !rock_stopped {
            if down {
                rock_stopped = try_move(&Direction::Down, rock.as_mut() ,&mut c, offset);
                offset += 1;
            } else {
                try_move(&jets[jet_i], rock.as_mut() ,&mut c, offset);
                jet_i = (jet_i + 1) % jets.len();
            }
            down = !down;
        }
        shape_i = (shape_i + 1) % SHAPES.len();
    }
    // println!("{}", c);
    c.layers.iter().filter(|a| **a != 0).count()
}

#[cfg(test)]

mod tests {
    use super::*;

    const INPUT: &str = "\
>>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
";

    #[test]
    fn test_one() {
        assert_eq!(solve(INPUT, 2022), 3068);
    }

    #[test]
    fn test_two() {
        assert_eq!(solve(INPUT, 1000000000000), 1514285714288);
    }
}