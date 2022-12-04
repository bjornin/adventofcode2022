use std::fs;
use std::collections::HashSet;

fn main() {
    let input = fs::read_to_string("day4/input.txt").unwrap();
    println!("{}", one(&input));
    println!("{}", two(&input));
}

fn one(input: &str) -> usize {
    let mut sum: usize = 0;
    for line in input.lines() {
        let g: Vec<usize> = line.split(|c| c == '-' || c == ',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        if (g[0] >= g[2] && g[1] <= g[3]) || (g[0] <= g[2] && g[1] >= g[3]) {
            sum += 1;
        }
    }
    sum
}

fn two(input: &str) -> usize {
    let mut sum: usize = 0;
    for line in input.lines() {
        let g: Vec<usize> = line.split(|c| c == '-' || c == ',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        let g1: HashSet<usize> = HashSet::from_iter(g[0]..=g[1]);
        let g2: HashSet<usize> = HashSet::from_iter(g[2]..=g[3]);
        if !g1.is_disjoint(&g2) {
            sum += 1;
        }
    }
    sum
}

#[cfg(test)]

mod tests {
    use super::*;
    
    const INPUT: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn test_one() {
        assert_eq!(one(INPUT), 2)
    }

    #[test]
    fn test_two() {
        assert_eq!(two(INPUT), 4)
    }
}