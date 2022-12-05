use std::collections::{HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
Part 1 From Youtube Comment:

    v.lines()
        .tuples()
        .into_iter()
        .map(|(l1, l2, l3)| l1.chars().find(|c| l2.contains(*c) && l3.contains(*c)))
        .map(Option::unwrap)
        .map(|c| {
            if c.is_lowercase() {
                c as i64 - 96
            } else {
                c as i64 - 38
            }
        })
        .sum() 
        
*/

pub fn run() -> (u32, u32) {

    let file_path = "src/day3/input.txt";

    let file = File::open(file_path).expect("Error in reading file");
    let lines = BufReader::new(file).lines();

    let mut total: u32 = 0;
    let mut total2: u32 = 0;

    let mut lineIndex = 0;

    let mut set21: HashSet<char> = HashSet::new();
    let mut set22: HashSet<char> = HashSet::new();
    let mut set23: HashSet<char> = HashSet::new();

    for line in lines {

        if let Ok(line) = line {

            let (comp1, comp2) = line.split_at(line.len()/2); 

            let mut set1 = HashSet::new();
            let mut set2 = HashSet::new();

            match lineIndex % 3 {
                0 => { 

                    for x in set21.intersection(&set22).find(|it| set23.contains(it)) {

                        let x = *x as u32;

                        if x >= 97 {

                            total2 += x - 96;
        
                        }
        
                        else {
        
                            total2 += x - 38;
        
                        }

                        //print!("{}", total2);

                    }

                    //println!();

                    set21.clear();
                    set22.clear();
                    set23.clear();
                                        
                    set21 = line.chars().collect(); 

                },
                1 => set22 = line.chars().collect(),
                2 => set23 = line.chars().collect(),
                _ => panic!("Error"),
            };

            for s in comp1.bytes() {

                set1.insert(s);

            }

            for s in comp2.bytes() {

                set2.insert(s);

            }

            let intersection: HashSet<_> = set1.intersection(&set2).collect();

            for x in intersection {

                let x = u32::from(*x);

                // lower case
                if x >= 97 {

                    total += x - 96;

                }

                else {

                    total += x - 38;

                }

            }

        }

        lineIndex += 1;

    }

    for x in set21.intersection(&set22).find(|it| set23.contains(it)) {

        let x = *x as u32;

        if x >= 97 {

            total2 += x - 96;

        }

        else {

            total2 += x - 38;

        }

    }

    (total, total2)

}