pub enum First {
    A, // Rock
    B, // Paper
    C, // Scissors
}

pub enum Second {
    X, //Rock 1pt
    Y, //Paper 2pt
    Z, //Scissors 3pt
}

pub fn parse (initial_contents : &String) -> Vec<(First, Second)> {
    use First::*;
    use Second::*;
    initial_contents.lines().map(|entry| {
        let first = match entry.chars().nth(0).unwrap() {
            'A' => A,
            'B' => B,
            'C' => C,
            _ => panic!("First failed"),
        };
        let second = match entry.chars().nth(2).unwrap() {
            'X' => X,
            'Y' => Y,
            'Z' => Z,
            _ => panic!("Second failed"),
        };
        (first, second)
    }).collect()
}

pub fn part1 (entry_data : Vec<(First, Second)>) -> i64 {
    use First::*;
    use Second::*;
    entry_data.iter().map(|(first, second)| {
        match (first, second) {
            (A, X) => 1+3,
            (B, X) => 1+0,
            (C, X) => 1+6,
            (A, Y) => 2+6,
            (B, Y) => 2+3,
            (C, Y) => 2+0,
            (A, Z) => 3+0,
            (B, Z) => 3+6,
            (C, Z) => 3+3,
        }
    }).sum()
}

pub fn part2 (entry_data : Vec<(First, Second)>) -> i64 {
    use First::*;
    use Second::*;
    entry_data.iter().map(|(first, second)| {
        match (first, second) {
            (A, X) => 3+0,
            (B, X) => 1+0,
            (C, X) => 2+0,
            (A, Y) => 1+3,
            (B, Y) => 2+3,
            (C, Y) => 3+3,
            (A, Z) => 2+6,
            (B, Z) => 3+6,
            (C, Z) => 1+6,
        }
    }).sum()
}