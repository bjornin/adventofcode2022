use std::fs;
use std::collections::{HashMap, HashSet};
use std::ops::{Add, Sub};

fn main() {
    let input = fs::read_to_string("day9/input.txt").unwrap();
    println!("{}", one(&input));
    println!("{}", two(&input));
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn distance(self, other: Coord) -> i32 {
        ((self.x - other.x).abs()).max((self.y - other.y).abs())
    }
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, other: Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Coord {
    type Output = Coord;

    fn sub(self, other: Coord) -> Coord {
        Coord {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

fn one(input: &str) -> usize {
    let mut head: Coord = Coord{x: 0, y: 0};
    let mut tail: Coord = Coord{x: 0, y: 0};
    let mut t_visit: HashSet<Coord> = HashSet::new();
    t_visit.insert(tail);
    let d = HashMap::from([
        ("R", Coord{x: 1, y: 0}),
        ("L", Coord{x: -1, y: 0}),
        ("U", Coord{x: 0, y: 1}),
        ("D", Coord{x: 0, y: -1}),
    ]);
    
    for line in input.lines() {
        let dir = line.split_whitespace().collect::<Vec<&str>>();
        let steps = dir[1].parse::<i32>().unwrap();
        for _ in 0..steps {
            let new_head = head + *d.get(dir[0]).unwrap();
            if new_head.distance(tail) > 1 {
                tail = head;
                t_visit.insert(tail);
            }
            head = new_head;
        }
    }
    t_visit.len()
}

fn two(input: &str) -> usize {
    let mut tail: Vec<Coord> = vec![Coord{x: 0, y: 0}; 10];
    let mut t_visit: HashSet<Coord> = HashSet::new();
    t_visit.insert(*tail.last().unwrap());
    let dir = HashMap::from([
        ("R", Coord{x: 1, y: 0}),
        ("L", Coord{x: -1, y: 0}),
        ("U", Coord{x: 0, y: 1}),
        ("D", Coord{x: 0, y: -1}),
    ]);
    
    for line in input.lines() {
        let d = line.split_whitespace().collect::<Vec<&str>>();
        let direction = *dir.get(d[0]).unwrap();
        let steps = d[1].parse::<i32>().unwrap();
        for _ in 0..steps {
            tail[0] = tail[0] + direction;
            for i in 1..tail.len() {
                if tail[i - 1].distance(tail[i]) > 1 {
                    let m = tail[i - 1] - tail[i];
                    tail[i] = tail[i] + Coord{x: m.x.signum(), y: m.y.signum()};
                }
            }
            t_visit.insert(*tail.last().unwrap());
        }
    }
    t_visit.len()
}

#[cfg(test)]

mod tests {
    use super::*;

    const INPUT: &str = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";
    #[test]
    fn test_one() {
        assert_eq!(one(INPUT), 13);
    }

    const INPUT2: &str = "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";
    #[test]
    fn test_two() {
        assert_eq!(two(INPUT), 1);
        assert_eq!(two(INPUT2), 36);
    }
}