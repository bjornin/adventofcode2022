use std::fs;
use std::collections::BTreeMap;


fn main() {
    let input = fs::read_to_string("day10/input.txt").unwrap();
    println!("{}", solve(&input));
}


fn solve(input: &str) -> i32 {
    let mut cycle = 1;
    let mut x = 1;
    let mut ret = BTreeMap::new();
    for line in input.lines() {
        let d = line.split_whitespace().collect::<Vec<&str>>();
        let instr = d[0];
        let num = if let Some(num) = d.get(1) {
            num.parse::<i32>().unwrap()
        } else {
            0
        };
        x += match instr {
            "noop" => {
                ret.insert(cycle, x);
                cycle += 1;
                0
            }
            "addx" => {
                for i in cycle..cycle+2 {
                    ret.insert(i, x);
                }
                cycle += 2;
                num
            }
            _ => panic!("unknown instruction")
        }
    }

    for (k, v) in &ret {
        let pos = (k - 1) % 40;
        if (v-1..=v+1).contains(&pos) {
            print!("#");
        } else {
            print!(".");
        }
        if pos == 39 {
            println!();
        }
    }

    (20..=220).step_by(40).map(|i| {
        i * ret.get(&i).unwrap()
    }).sum()

}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(one(INPUT2), 13140);
    }

    const INPUT: &str = "\
noop
addx 3
addx -5
noop
";
    const INPUT2: &str = "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";
}