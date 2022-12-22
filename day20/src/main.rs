use std::{fs, collections::VecDeque};

fn main() {
    let input = fs::read_to_string("day20/input.txt").unwrap();
    println!("{}", one(&input));
    println!("{}", two(&input));
}

fn one(input: &str) -> i64 {
    let mut num = parse(input, 1);
    mix(&mut num);

    let Some(z) = num.iter().position(|&(_, v)| v == 0) else {panic!("no z")};
    num.iter().cycle().nth(1000+z).unwrap().1 +
    num.iter().cycle().nth(2000+z).unwrap().1 +
    num.iter().cycle().nth(3000+z).unwrap().1
}

fn two(input: &str) -> i64 {
    let mut num = parse(input, 811589153);

    for _ in 0..10 {
        mix(&mut num);
    }

    let Some(z) = num.iter().position(|&(_, v)| v == 0) else {panic!("no z")};
    num.iter().cycle().nth(1000+z).unwrap().1 +
    num.iter().cycle().nth(2000+z).unwrap().1 +
    num.iter().cycle().nth(3000+z).unwrap().1
}

fn mix(num: &mut VecDeque<(i64, i64)>) {
    let num_len = num.len() as i64;
    for i in 0..num_len {
        let Some(e) = num.iter()
            .position(|(j,_)| *j == i) else { panic!("nope") };
        let Some(v) = num.remove(e) else { panic!("no rem") };
        let ins = get_index(e, v, num_len);
        num.insert(ins as usize, v);
    }
}

fn parse(input: &str, key: i64) -> VecDeque<(i64, i64)> {
    let num: VecDeque<(i64, i64)> = input.lines().enumerate()
        .map(|(i, x)| (i as i64, key * x.parse::<i64>().unwrap())).collect();
    num
}

fn get_index(e: usize, v: (i64, i64), num_len: i64) -> i64 {
    let b = num_len - 1;
    (((e as i64 + v.1) % b) + b) % b // negative modulo hack
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(one(INPUT), 3);
    }

    #[test]
    fn test_two() {
        assert_eq!(two(INPUT), 1623178306);
    }

    #[test]
    fn test_index() {
        // [1,0,0,0,0] -> [0,1,0,0,0]
        assert_eq!(get_index(0, (0,1), 5), 1);
        // [-1,0,0,0,0] -> [0,0,0,-1,0]
        assert_eq!(get_index(0, (0,-1), 5), 3);
        // [-3,0,0] -> [0,-3,0]
        assert_eq!(get_index(0, (0,-3), 3), 1);
        // [3,0,0] -> [0,3,0] 4
        assert_eq!(get_index(0, (0,3), 3), 1);
        // [-6,0,0] -> [-6,0,0]
        assert_eq!(get_index(0, (0,-6), 3), 0);
        // [-8,0,0,0] -> [0,-8,0,0]
        assert_eq!(get_index(0, (0,-8), 4), 1);
        // [8,0,0,0] -> [0,0,8,0]
        assert_eq!(get_index(0, (0,8), 4), 2);
        // [11,0,0,0] -> [0,0,8,0]
        assert_eq!(get_index(0, (0,11), 4), 2);
        // [-10,0,0,0,0] -> [0,0,-10,0,0]
        assert_eq!(get_index(0, (0,-10), 5), 2);
        // [-15,0,0,0,0,0] -> [-15,0,0,0,0,0]
        assert_eq!(get_index(0, (0,-15), 6), 0);
        // [9,0,0] -> [0,9,0]
        assert_eq!(get_index(0, (0,9), 3), 1);
        // [-9,0,0] -> [0,-9,0]
        assert_eq!(get_index(0, (0,-9), 3), 1);
    }

    const INPUT: &str = "\
1
2
-3
3
-2
0
4
";



}