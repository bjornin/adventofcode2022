use std::fs;
use std::convert::From;
use std::ops::Add;

fn main() {
    let input = fs::read_to_string("day18/input.txt").unwrap();
    println!("{}", one(&input));
    // println!("{}", two(&input));
}

#[derive(Copy, Clone, PartialEq)]
struct Pixel {
    x: i32,
    y: i32,
    z: i32,
}

impl Pixel {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Pixel { x, y, z }
    }
}

impl Add for Pixel {
    type Output = Pixel;

    fn add(self, rhs: Self) -> Self::Output {
        Pixel {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl From<&str> for Pixel {
    fn from(value: &str) -> Self {
        let mut n = value.split(',');
        Pixel {
            x: n.next().unwrap().parse::<i32>().unwrap(),
            y: n.next().unwrap().parse::<i32>().unwrap(),
            z: n.next().unwrap().parse::<i32>().unwrap(),
        }
    }
}


fn one(input: &str) -> usize {
    let pixels: Vec<Pixel> = input.lines()
        .map(Pixel::from)
        .collect();

    let directions: Vec<Pixel> = vec![
        Pixel::new(0, 0, 1),
        Pixel::new(0, 0, -1),
        Pixel::new(0, 1, 0),
        Pixel::new(0, -1, 0),
        Pixel::new(1, 0, 0),
        Pixel::new(-1, 0, 0),
    ];

    let mut area = 0;
    for p in pixels.iter() {
        for d in &directions {
            let pn = *p + *d;
            if !pixels.contains(&pn) {
                area += 1;
            }
        }
    }
    area
}

// fn two(input: &str) -> usize {

// }

#[cfg(test)]

mod tests {
    use super::*;

    const INPUT: &str = "\
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

    #[test]
    fn test_one() {
        assert_eq!(one(INPUT), 64);
    }

    // #[test]
    // fn test_two() {
    //     assert_eq!(two(INPUT), );
    // }
}