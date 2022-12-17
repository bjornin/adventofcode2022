use std::collections::hash_map::Entry::{Vacant, Occupied};
use std::{fs, collections::HashMap};
use std::convert::From;

#[derive(Debug, PartialEq)]
enum Tile {
    Sand,
    Rock,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    c: i32,
    r: i32,
}

impl Point {
    fn new(c: i32, r: i32) -> Self {
        Point {c ,r}
    }

    fn over(&self) -> Self {
        Point {
            c: self.c,
            r: self.r - 1,
        }
    }
    fn left(&self) -> Self {
        Point {
            c: self.c - 1,
            r: self.r,
        }
    }
    fn right(&self) -> Self {
        Point {
            c: self.c + 1,
            r: self.r,
        }
    }
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let mut p = value.split(",");
        let c = p.next().unwrap().parse::<i32>().unwrap();
        let r = p.next().unwrap().parse::<i32>().unwrap();

        Point { c, r }
    }
}

#[derive(Debug)]
struct Cave {
    scan: HashMap<Point, Tile>,
    floor: i32,
}

impl Cave {
    fn floor(&mut self, at: i32) {
        self.floor = at;
    }
}

impl From<&str> for Cave {
    fn from(value: &str) -> Self {
        let mut scan: HashMap<Point, Tile> = HashMap::new();
        for line in value.lines() {
            for w in line
                .split(" -> ")
                .map(Point::from)
                .collect::<Vec<Point>>()
                .windows(2)
            {
                let c_start = w[0].c.min(w[1].c);
                let c_end = w[0].c.max(w[1].c);
                let r_start = w[0].r.min(w[1].r);
                let r_end = w[0].r.max(w[1].r);
                for c in c_start..=c_end {
                    for r in r_start..=r_end {
                        scan.insert(Point { c, r }, Tile::Rock);
                    }
                }
            }
        }

        Cave {
            scan,
            floor: -1
        }
    }
}

fn main() {
    let input = fs::read_to_string("day14/input.txt").unwrap();
    println!("{}", one(&input));
    println!("{}", two(&input));
}

fn one(input: &str) -> usize {
    let mut cave = Cave::from(input);
    let mut sand = Point::new(500,0);
    loop {
        let Some(n) = cave.scan
            .iter()
            .filter(|(p, _)| p.c == sand.c && p.r > sand.r)
            .min_by_key(|(p, _)| p.r) else {
                break
            };
        let p = *n.0;
        match cave.scan.entry(p.left()) {
            Vacant(v) => {
                sand = v.into_key();
                continue;
            }
            Occupied(_) => (),
        };
        match cave.scan.entry(p.right()) {
            Vacant(v) => {
                sand = v.into_key();
                continue;
            }
            Occupied(_) => {
                cave.scan.insert(p.over(), Tile::Sand);
                sand = Point::new(500,0);
            }
        };   
    }
    cave.scan.into_iter().filter(|(_, t)| t == &Tile::Sand).count()
}

fn two(input: &str) -> usize {
    let mut cave = Cave::from(input);
    let f = if let Some(f) = cave.scan
        .iter()
        .max_by_key(|(p, _)| p.r) {
            cave.floor(f.0.r + 2);
        };
    let mut sand = Point::new(500,0);
    loop {
        if let Some(Tile::Sand) = cave.scan.get(&sand) {
            break;
        }
        let p = match cave.scan
            .iter()
            .filter(|(p, _)| p.c == sand.c && p.r > sand.r)
            .min_by_key(|(p, _)| p.r) {
                Some(n) => *n.0,
                _ => Point::new(sand.c, cave.floor),
            };
        if p.r == cave.floor {
            cave.scan.insert(p.over(), Tile::Sand);
            sand = Point::new(500,0);
            continue;
        }
        match cave.scan.entry(p.left()) {
            Vacant(v) => {
                sand = v.into_key();
                continue;
            }
            Occupied(_) => (),
        };
        match cave.scan.entry(p.right()) {
            Vacant(v) => {
                sand = v.into_key();
                continue;
            }
            Occupied(_) => {
                cave.scan.insert(p.over(), Tile::Sand);
                sand = Point::new(500,0);
            }
        };
    }
    cave.scan.into_iter().filter(|(_, t)| t == &Tile::Sand).count()
}

#[cfg(test)]

mod tests {
    use super::*;

    const INPUT: &str = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

    #[test]
    fn test_one() {
        assert_eq!(one(INPUT), 24)
    }

    #[test]
    fn test_two() {
        assert_eq!(two(INPUT), 93)
    }
}