use std::fs;
use std::collections::{HashMap, HashSet};
use regex::Regex;

#[derive(Clone, Eq, PartialEq, Debug)]
enum Mark {
    Sensor,
    Beacon,
    None,
    Unknown,
}

#[derive(Debug)]
struct Sensor {
    x: i64,
    y: i64,
    dist: i64,
}

impl Sensor {
    fn new(x: i64, y: i64, bx: i64, by: i64) -> Self {
        let dist = dist(x, y, bx, by);
        Sensor { x, y, dist }
    }
    fn perimiter(&self) -> HashSet<(i64, i64)> {
        let mut p: HashSet<(i64, i64)> = HashSet::new();
        let r1 = self.y-self.dist-1..=self.y;
        let r2 = self.x..=self.x+self.dist+1;
        let r3 = self.y..=self.y+self.dist+1;
        let r4 = self.x-self.dist-1..=self.x;
        // let i1 = r1.zip(r2);
        p.extend(r1.clone().zip(r2.clone()));
        p.extend(r2.rev().zip(r3.clone()));
        p.extend(r3.zip(r4.clone().rev()));
        p.extend(r4.rev().zip(r1.rev()));
        // p.append(Vec::from(i1));
        p
        // x8,y7
        // y -3..=7
        // x 8..=18
        // y 7..17
        // x 8..=-2
        // x_range.zip(&y_range)
    }
}

fn dist(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn main() {
    let input = fs::read_to_string("day15/input.txt").unwrap();
    println!("{}", one(&input, 2000000));
    println!("{}", two(&input, 4000000));
}

fn one(input: &str, row: i64) -> usize {
    let re: Regex = Regex::new(r"-?\d+").unwrap();

    let mut sensors: Vec<Sensor> = Vec::new();
    let mut marks: HashMap<(i64, i64), Mark> = HashMap::new();
    for line in input.lines() {
        let [x, y, bx, by]: [i64; 4] = re.find_iter(line)
            .filter_map(|d| d.as_str().parse::<i64>().ok())
            .collect::<Vec<i64>>().try_into().unwrap();
        sensors.push(Sensor::new(x, y, bx, by));
        marks.insert((bx, by), Mark::Beacon);
        marks.insert((x, y), Mark::Sensor);
    }
    for (s, n) in sensors.into_iter()
        .filter_map(|t| {
            let n = (t.y - row).abs();
            if n <= t.dist {
                Some((t, n))
            } else {
                None
            }
        })
    {
        let range = s.x - (s.dist - n)..=s.x + (s.dist - n);
        for x in range {
            marks.entry((x, row)).or_insert(Mark::None);
        }
    }
    marks.into_iter().filter(|(_, m)| m == &Mark::None).count()
}

fn two(input: &str, max_c: i64) -> usize {
    let re: Regex = Regex::new(r"-?\d+").unwrap();

    let mut sensors: Vec<Sensor> = Vec::new();
    let mut marks: HashMap<(i64, i64), Mark> = HashMap::new();
    for line in input.lines() {
        let [x, y, bx, by]: [i64; 4] = re.find_iter(line)
            .filter_map(|d| d.as_str().parse::<i64>().ok())
            .collect::<Vec<i64>>().try_into().unwrap();
        let s = Sensor::new(x, y, bx, by);
        for (px, py) in s.perimiter() {
            if px >=0 && px <= max_c && py >= 0 && py <= max_c {
                marks.insert((px, py), Mark::Unknown);
            }
        }
        sensors.push(s);
    }
    for (x, y) in marks.clone().keys() {
        for s in sensors.iter() {
            if dist(*x, *y, (*s).x, (*s).y) <= (*s).dist {
                marks.insert((*x, *y), Mark::None);
            }
        }
    }
    for ((x, y), _) in marks.into_iter()
        .filter(|(_, m)| m == &Mark::Unknown)
    {
        return (x * 4000000 + y).try_into().unwrap()
    }
    0
}

#[cfg(test)]

mod tests {
    use super::*;

    const INPUT: &str = "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3    
";

    #[test]
    fn test_one() {
        assert_eq!(one(INPUT, 10), 26)
    }

    #[test]
    fn test_two() {
        assert_eq!(two(INPUT, 20), 56000011)
    }
}