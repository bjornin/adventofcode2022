use std::{fs, collections::VecDeque};

fn main() {
    let input = fs::read_to_string("day20/input.txt").unwrap();
    println!("{}", one(&input));
    // println!("{}", two(&input));
}

fn one(input: &str) -> i32 {
    let num = mix(input);

    let Some(z) = num.iter().position(|&(_, v)| v == 0) else {panic!("no z")};
    num.iter().cycle().nth(1000+z).unwrap().1 +
    num.iter().cycle().nth(2000+z).unwrap().1 +
    num.iter().cycle().nth(3000+z).unwrap().1
}

fn mix(input: &str) -> VecDeque<(i32, i32)> {
    let mut num: VecDeque<(i32, i32)> = input.lines().enumerate().map(|(i, x)| (i as i32, x.parse::<i32>().unwrap())).collect();
    let num_len = num.len() as i32;
    for i in 0..num_len {
        let Some(e) = num.iter()
            .position(|(j,_)| *j == i) else { panic!("nope") };
        let Some(v) = num.remove(e) else { panic!("no rem") };
    
        // let idx = get_index(e, v, num_len);
        let ins = get_index(e, v, num_len);

        // println!("moving i:{} e:{} ins:{} v:{:?}", i, e, ins, v);
        num.insert(ins as usize, v);
        // println!("{:?}", num);
    }
    num
}

fn get_index(e: usize, v: (i32, i32), num_len: i32) -> i32 {
    let b = num_len - 1;
    (((e as i32 + v.1) % b) + b) % b
}

// fn two(input: &str) -> usize {

// }

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(one(INPUT), 3);
    }

    // #[test]
    // fn test_two() {
    //     assert_eq!(two(INPUT), );
    // }

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