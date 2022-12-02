use std::fs;

fn main() {
    let input = fs::read_to_string("day2/input.txt").unwrap();
    println!("{}", one(&input));
    println!("{}", two(&input));
}

fn play1(p1: &str, p2: &str) -> usize {
    let mut sum: usize = 0;
    if (p1 == "A" && p2 == "Y") ||
    (p1 == "B" && p2 == "Z") ||
    (p1 == "C" && p2 == "X") {
        sum += 6;
    } else if (p1 == "A" && p2 == "X") ||
    (p1 == "B" && p2 == "Y") ||
    (p1 == "C" && p2 == "Z") {
        sum += 3;
    }
    if p2 == "X" {
        sum += 1;
    } else if p2 == "Y" {
        sum += 2;
    } else if p2 == "Z" {
        sum += 3;
    }
    sum
}

fn play2(p1: &str, res: &str) -> usize {
    let mut sum: usize = 0;
    if p1 == "A" {
        if res == "X" {
            sum += 3;
        } else if res == "Y" {
            sum += 1 + 3;
        } else {
            sum += 2 + 6;
        }
    } else if p1 == "B" {
        if res == "X" {
            sum += 1;
        } else if res == "Y" {
            sum += 2 + 3;
        } else {
            sum += 3 + 6;
        }
    } else if p1 == "C" {
        if res == "X" {
            sum += 2;
        } else if res == "Y" {
            sum += 3 + 3;
        } else {
            sum += 1 + 6;
        }
    }
    sum
}

fn one(input: &str) -> usize {
    let mut sum: usize = 0;
    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let opp = iter.next().unwrap();
        let me = iter.next().unwrap();
        sum += play1(opp, me);
    }
    sum
}

fn two(input: &str) -> usize {
    let mut sum: usize = 0;
    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let opp = iter.next().unwrap();
        let me = iter.next().unwrap();
        sum += play2(opp, me);
    }
    sum
}

#[cfg(test)]

mod tests {
    use super::*;

    const INPUT: &str = "\
A Y
B X
C Z
";

    #[test]
    fn play1() {
        assert_eq!(one(INPUT), 15)
    }

    #[test]
    fn play2() {
        assert_eq!(two(INPUT), 12)
    }
}