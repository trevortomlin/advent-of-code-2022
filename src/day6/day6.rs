use std::{fs, collections::VecDeque};

pub fn run() -> (u32, u32) {

    let input = fs::read_to_string("src/day5/input.txt")
    .expect("Should have been able to read the file");

    let p1 = part1(&input);
    let p2 = part2(&input);

    return (p1, p2);

}

fn part1(input: &str) -> u32 {

    // input.lines()
    //         .split("\n")
    //         .filter()

    2

}

fn part2(input: &str) -> u32 {

    2

}