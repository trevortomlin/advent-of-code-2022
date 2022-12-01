use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() -> (u32, u32) {

    let file_path = "src/day1/input.txt";

    let file = File::open(file_path).expect("Error in reading file");
    let lines = BufReader::new(file).lines();

    let mut res = Vec::new();

    let mut sum = 0;

    for line in lines {

        if let Ok(line) = line {

            if line.len() == 0 {

                res.push(sum);
                sum = 0;

            }

            else {

                sum += line.parse::<u32>().unwrap();

            }

        }

    }

    res.sort();

    let mut top3 = 0;

    for i in 0..3 {

        top3 += res[res.len() - 1 - i];

    }

    return (*res.iter().max().unwrap(), top3);

}
