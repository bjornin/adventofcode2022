#![feature(iter_next_chunk)]

use std::{fs, collections::HashMap};

fn main() {
    let input = fs::read_to_string("day3/input.txt").unwrap();
    println!("{}", one(&input));
    println!("{}", two(&input));
}

fn counter(s: &str) -> HashMap<char, i32> {
    let mut map = HashMap::new();
    for c in s.chars() {
        *map.entry(c).or_insert(0) += 1;
    }
    map
}

fn one(input: &str) -> usize {
    let mut sum = 0;

    for sack in input.lines() {
        let middle = sack.len() / 2;
        let comp1 = &sack[0..middle];
        let comp2 = &sack[middle..];
        let counter1 = counter(comp1);
        let counter2 = counter(comp2);
        let i = counter1
            .iter()
            .filter(|(c, _)| counter2.contains_key(c))
            .map(|(c, _)| c)
            .collect::<String>();
        let it = i.chars().next().unwrap();
        sum += Item::new(it).prio();
    }
    sum
}


fn two(input: &str) -> usize {
    let mut sum = 0;
    let mut iter = input.lines();
    while let Ok(group) = iter.next_chunk::<3>() {
        let counter1 = counter(group[0]);
        let counter2 = counter(group[1]);
        let counter3 = counter(group[2]);
        let i = counter1
            .iter()
            .filter(|(c, _)| counter2.contains_key(c))
            .filter(|(c, _)| counter3.contains_key(c))
            .map(|(c, _)| c)
            .collect::<String>();
        let it = i.chars().next().unwrap();
        sum += Item::new(it).prio();
    }
    sum
}

pub struct Item {
    c: char
}

impl Item {
    pub fn new (c: char) -> Item {
        Item {
            c
        }
    }

    pub fn prio(&self) -> usize {
        if self.c.is_lowercase() {
            self.c as usize - 'a' as usize + 1
        } else {
            self.c as usize - 'A' as usize + 27
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    const INPUT: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

    #[test]
    fn test_one() {
        assert_eq!(one(INPUT), 157);
    }

    #[test]
    fn test_two() {
        assert_eq!(two(INPUT), 70);
    }
}