use std::{fs, collections::HashSet};

fn main() {
    let input = fs::read_to_string("day9/input.txt").unwrap();
    println!("{}", one(&input));
    println!("{}", two(&input));
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Coord(i32, i32);

fn update_t(t: Coord, h: Coord) -> Coord {
    let mut x = t.0;
    let mut y = t.1;
    let dx = h.0 - t.0;
    let dy = h.1 - t.1;
    if dx.abs() + dy.abs() > 2 {
        x += dx / dx.abs();
        y += dy / dy.abs();
    } else if !(-1..=1).contains(&dx) {
        x += dx / dx.abs();
    } else if !(-1..=1).contains(&dy) {
        y += dy / dy.abs();
    }
    Coord(x, y)
}

fn one(input: &str) -> usize {
    let mut h_pos: Coord = Coord(0,0);
    let mut t_pos: Coord = Coord(0,0);
    let mut t_visit: HashSet<Coord> = HashSet::new();
    t_visit.insert(t_pos);
    
    for line in input.lines() {
        let dir = line.split_whitespace().collect::<Vec<&str>>();
        let steps = dir[1].parse::<i32>().unwrap();
        match dir[0] {
            "R" => {
                for _ in 0..steps {
                    h_pos = Coord(h_pos.0 + 1, h_pos.1);
                    t_pos = update_t(t_pos, h_pos);
                    t_visit.insert(t_pos);
                }
            }
            "L" => {
                for _ in 0..steps {
                    h_pos = Coord(h_pos.0 - 1, h_pos.1);
                    t_pos = update_t(t_pos, h_pos);
                    t_visit.insert(t_pos);
                }
            }
            "U" => {
                for _ in 0..steps {
                    h_pos = Coord(h_pos.0, h_pos.1 + 1);
                    t_pos = update_t(t_pos, h_pos);
                    t_visit.insert(t_pos);
                }
            }
            "D" => {
                for _ in 0..steps {
                    h_pos = Coord(h_pos.0, h_pos.1 - 1);
                    t_pos = update_t(t_pos, h_pos);
                    t_visit.insert(t_pos);
                }
            }
            &_ => println!("odd move")
        }
    }
    t_visit.len()
}

fn two(input: &str) -> usize {
    36
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
        assert_eq!(two(INPUT), 0);
        assert_eq!(two(INPUT2), 36);
    }
}