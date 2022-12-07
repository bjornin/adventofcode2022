#![feature(iter_advance_by)]

use std::fs;

fn main() {
    let input = fs::read_to_string("day6/input.txt").unwrap();
    println!("{}", one(&input));
    println!("{}", two(&input));
}

fn one(input: &str) -> usize {
    let mut ret = 0;
    let mut iter = input.chars().enumerate();
    iter.advance_by(3).unwrap();
    while let Some((i, c)) = iter.next() {
        let window = &input[(i-3)..=i];
        if window.chars().all(|x| (&window).match_indices(x).collect::<Vec<_>>().len() == 1) {
            ret = i + 1;
            break;
        }
    }
    ret
}

fn two(input: &str) -> usize {
    let mut ret = 0;
    let mut iter = input.chars().enumerate();
    iter.advance_by(13).unwrap();
    while let Some((i, c)) = iter.next() {
        let window = &input[(i-13)..=i];
        if window.chars().all(|x| (&window).match_indices(x).collect::<Vec<_>>().len() == 1) {
            ret = i + 1;
            break;
        }
    }
    ret
}

#[cfg(test)]

mod tests {
    use super::*;

    const INPUT: &str = "\
mjqjpqmgbljsphdztnvjfqwrcgsmlb
";

    #[test]
    fn test_one() {
        assert_eq!(one(INPUT), 7)
    }

    #[test]
    fn test_two() {
        assert_eq!(two(INPUT), 19)
    }
}