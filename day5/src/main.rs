use std::{fs, collections::HashMap, vec};
use regex::Regex;

struct Move(i32,i32,i32);

fn main() {
    let input = fs::read_to_string("day5/input.txt").unwrap();
    println!("{}", one(&input));
    println!("{}", two(&input));
}

fn one(input: &str) -> String {
    let mut stacks: HashMap<i32, Vec<char>> = parse_stacks(input);
    let moves: Vec<Move> = parse_moves(input);
    for m in moves {
        for _ in 0..m.0 {
            let thing = stacks.get_mut(&m.1).unwrap().pop().unwrap();
            stacks.get_mut(&m.2).unwrap().push(thing);
        }
    }
    get_top_crates(stacks)
}

fn two(input: &str) -> String {
    let mut stacks: HashMap<i32, Vec<char>> = parse_stacks(input);
    let moves: Vec<Move> = parse_moves(input);
    for m in moves {
        let len = stacks.get(&m.1).unwrap().len();
        for _ in 0..m.0 {
            let c = stacks.get_mut(&m.1).unwrap().remove(len - (m.0 as usize));
            stacks.get_mut(&m.2).unwrap().push(c);
        }
    }
    get_top_crates(stacks)
}

fn parse_stacks(input: &str) -> HashMap<i32, Vec<char>> {
    let mut stack_string: String = String::new();
    let mut num_stacks: i32 = 0;
    for line in input.lines() {
        if line.contains('1') {
            num_stacks = line.chars().map(|v| v.to_digit(10).unwrap_or(0)).max().unwrap_or(0) as i32;
            break;
        }
        stack_string.push_str(line
                .chars()
                .enumerate()
                .filter(|(i, _)| i % 2 == 1)
                .map(|(_, v)| v)
                .enumerate()
                .filter(|(i, _)| i % 2 == 0)
                .map(|(_, v)| v)
                .collect::<String>().as_str()
        );
    }
    let mut ret = HashMap::new();
    for (i, c) in stack_string.chars().rev().enumerate().filter(|(_, c)| *c != ' ') {
        let ind: i32 = (i as i32 % num_stacks - num_stacks).abs();
        ret.entry(ind).and_modify(|v: &mut Vec<char>| v.push(c)).or_insert_with(|| vec![c]);
    }
    ret
}

fn parse_moves(input: &str) -> Vec<Move> {
    let re = Regex::new(r"move (?P<count>\d+).*(?P<from>\d+).*(?P<to>\d+)").unwrap();
    let mut ret: Vec<Move> = Vec::new();
    for line in input.lines() {
        let cap = re.captures(line);
        match cap {
            Some(c) => {
                ret.push(Move(
                    c["count"].parse::<i32>().unwrap(),
                    c["from"].parse::<i32>().unwrap(),
                    c["to"].parse::<i32>().unwrap()),
                );
            }
            None => continue,
        }
    }
    ret
}

fn get_top_crates(mut stacks: HashMap<i32, Vec<char>>) -> String {
    let mut ret = String::new();
    for i in 1..=stacks.len() as i32 {
        ret.push(stacks.get_mut(&i).unwrap().pop().unwrap());
    }
    ret
}

#[cfg(test)]

mod tests {
    use super::*;

    const INPUT: &str = r#"
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

01234567890
ssss[d]ssss
 s s d s s
 s   d   s

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;

    #[test]
    fn test_one() {
        assert_eq!(one(INPUT), "CMZ")
    }

    #[test]
    fn test_two() {
        assert_eq!(two(INPUT), "MCD")
    }

}