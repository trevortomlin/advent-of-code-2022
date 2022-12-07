use std::{fs, collections::{VecDeque, HashSet}, hash::Hash};

pub fn run() -> (u32, u32) {

    let input = fs::read_to_string("src/day6/input.txt")
    .expect("Should have been able to read the file");

    let p1 = part1(&input);
    let p2 = part2(&input);

    return (p1, p2);

}

fn part1(input: &str) -> u32 {

    for line in input.lines() {

        let mut s = HashSet::new();

        let mut l = 0;
        let mut r = 4;

        while r < line.len() {

            for i in l..r {

                s.insert(line.as_bytes()[i]);

            }

            if s.len() == 4 {

                return r as u32;

            }

            s.clear();

            l += 1;
            r += 1;

        }

    }

    0      

}

fn part2(input: &str) -> u32 {

    for line in input.lines() {

        let mut s = HashSet::new();

        let mut l = 0;
        let mut r = 14;

        while r < line.len() {

            for i in l..r {

                s.insert(line.as_bytes()[i]);

            }

            if s.len() == 14 {

                return r as u32;

            }

            s.clear();

            l += 1;
            r += 1;

        }

    }

    0      

}