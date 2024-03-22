use std::fs;

fn main() {
    let input = fs::read_to_string("dayX/input.txt").unwrap();
    println!("{}", one(&input));
    // println!("{}", two(&input));
}

fn one(input: &str) -> usize {

}

// fn two(input: &str) -> usize {

// }

#[cfg(test)]

mod tests {
    use super::*;

    const INPUT: &str = "\

";

    #[test]
    fn test_one() {
        assert_eq!(one(INPUT), );
    }

    // #[test]
    // fn test_two() {
    //     assert_eq!(two(INPUT), );
    // }
}