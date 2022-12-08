use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("day8/input.txt").unwrap();
    println!("{}", one(&input));
    println!("{}", two(&input));
}

fn check_visibility(pos: usize, list: &Vec<usize>) -> usize {
    let ret = pos == 0 ||
                    pos == list.len() - 1 ||
                    list[pos] > *(list[..pos].iter().max().unwrap()) ||
                    list[pos] > *(list[pos+1..].iter().max().unwrap());
    ret as usize
}

fn one(input: &str) -> usize {
    let mut v: Vec<Vec<usize>> = Vec::new();
    let mut visible: HashMap<(usize, usize), usize> = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        if v.is_empty() {
            (0..line.len()).for_each(|_| {
                v.push(vec![]);
            })
        }
        for (j, c) in line.chars().enumerate() {
            v.get_mut(j).unwrap().push(c.to_digit(10).unwrap() as usize);
            let tree = check_visibility(j, &line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect());
            *visible.entry((i, j)).or_insert(0) += tree;
        }
    }

    for (k, r) in v.into_iter().enumerate() {
        for j in 0..r.len() {
            let tree = check_visibility(j, &r);
            *visible.entry((j, k)).or_insert(0) += tree;
        }
    }
    let ret = visible.iter().filter(|(_, &v)| v > 0).count();
    ret
}

fn check_distance(pos: usize, list: &Vec<usize>) -> usize {
    let mut r = 0;
    let mut l = 0;
    let cur = list[pos];
    for &v in list[pos+1..].iter() {
        if v < cur {
            r += 1;
        } else if v >= cur {
            r += 1;
            break;
        }
    }
    for &v in list[..pos].iter().rev() {
        if v < cur {
            l += 1;
        } else if v >= cur {
            l += 1;
            break;
        }
    }
    r * l
}

fn two(input: &str) -> usize {
    let mut v: Vec<Vec<usize>> = Vec::new();
    let mut visible: HashMap<(usize, usize), usize> = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        if v.is_empty() {
            (0..line.len()).for_each(|_| {
                v.push(vec![]);
            })
        }
        for (j, c) in line.chars().enumerate() {
            v.get_mut(j).unwrap().push(c.to_digit(10).unwrap() as usize);
            let tree = check_distance(j, &line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect());
            *visible.entry((i, j)).or_insert(1) *= tree;
        }
    }
    
    for (k, r) in v.into_iter().enumerate() {
        for j in 0..r.len() {
            let tree = check_distance(j, &r);
            *visible.entry((j, k)).or_insert(1) *= tree;
        }
    }
    // println!("{:?}", visible);
    let ret = visible.iter()
        .map(|(_, &v)| v)
        .max().unwrap();
    ret
}

#[cfg(test)]

mod tests {
    use super::*;

    const INPUT: &str = "\
30373
25512
65332
33549
35390
";

    #[test]
    fn test_one() {
        assert_eq!(one(INPUT), 21);
    }

    #[test]
    fn test_two() {
        assert_eq!(two(INPUT), 8)
    }
}