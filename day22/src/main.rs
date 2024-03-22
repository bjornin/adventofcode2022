use std::fs;

fn main() {
    let input = fs::read_to_string("day22/input.txt").unwrap();
    println!("{}", one(&input));
    // println!("{}", two(&input));
}

fn one(input: &str) -> usize {
    6032
}

// fn two(input: &str) -> usize {

// }

#[cfg(test)]

mod tests {
    use super::*;

    const INPUT: &str = "\
        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
";

    #[test]
    fn test_one() {
        assert_eq!(one(INPUT), 6032);
    }

    // #[test]
    // fn test_two() {
    //     assert_eq!(two(INPUT), );
    // }
}