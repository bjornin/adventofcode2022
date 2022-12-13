use std::collections::{VecDeque, HashSet};
use std::fs;
use std::convert::From;
use std::ops::Add;

fn main() {
    let input = fs::read_to_string("day12/input.txt").unwrap();
    println!("{}", one(&input));
    println!("{}", two(&input));
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

}

impl Add for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Map {
    data: Vec<Vec<usize>>,
    start: Point,
    end: Point,
}

impl Map {
    fn height(&self, p: Point) -> usize {
        // self.data.get(p.x as usize).get(p.y as usize)
        self.data[p.y as usize][p.x as usize]
    }
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let mut data = Vec::new();
        let mut start = Point::new(0, 0);
        let mut end = Point::new(0, 0);
        
        for (i, r) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (j, c) in r.chars().enumerate() {
                match c {
                    'S' => {
                        row.push('a' as usize);
                        start = Point::new(j as i32, i as i32);
                    }
                    'E' => {
                        row.push('z' as usize);
                        end = Point::new(j as i32, i as i32);
                    }
                    _ => row.push(c as usize),
                }
            }
            data.push(row);
        }

        Map {
            data,
            start,
            end,
        }
    }
}

fn one(input: &str) -> usize {

    let map = Map::from(input);
    solve(&map, map.start)
}

fn two(input: &str) -> usize {
    let map = Map::from(input);
    let mut ret = Vec::new();
    for (i, iv) in map.data.iter().enumerate() {
        for (j, jv) in iv.iter().enumerate() {
            if ('a' as usize).eq(jv) {
                ret.push(solve(&map, Point::new(j as i32, i as i32)));
            }
        }
    }
    *ret.iter().filter(|&x| x > &0).min().unwrap()
}

fn solve(m: &Map, start: Point) -> usize {
    let mut q: VecDeque<(Point, usize)> = VecDeque::new();
    let mut visited: HashSet<Point> = HashSet::new();
    q.push_back((start, 0));

    while !q.is_empty() {
        let (current, dist) = q.pop_front().unwrap();
        if !visited.insert(current) {
            continue;
        }
        if current.eq(&(*m).end) {
            return dist
        }
        for i in [
            Point::new(0,1),
            Point::new(0,-1),
            Point::new(1,0),
            Point::new(-1,0)]
        {
            let n = current + i;
            if n.x >= 0
                && n.y >= 0
                && n.x < m.data[0].len() as i32
                && n.y < m.data.len() as i32
                && !visited.contains(&n)
                && m.height(n) <= m.height(current) + 1
            {
                q.push_back((n, dist + 1));
            }
        }
    }
    0
}

#[cfg(test)]

mod tests {
    use super::*;

    const INPUT: &str = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

    #[test]
    fn test_one() {
        assert_eq!(one(INPUT), 31)
    }

    #[test]
    fn test_two() {
        assert_eq!(two(INPUT), 29)
    }
}
