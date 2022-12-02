use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() -> (u32, u32) {

    let file_path = "src/day2/input.txt";

    let file = File::open(file_path).expect("Error in reading file");
    let lines = BufReader::new(file).lines();

    let mut res: Vec<(String, String)> = Vec::new();

    for line in lines {

        if let Ok(line) = line {

            let opponent_choice = (line.as_bytes()[0] as char).to_string();
            let your_choice = (line.as_bytes()[2] as char).to_string();

            let tuple = (opponent_choice, your_choice);

            res.push(tuple);

        }

    }

    let mut map = HashMap::new();

    map.insert("A".to_string(), 1);
    map.insert("B".to_string(), 2);
    map.insert("C".to_string(), 3);
    map.insert("X".to_string(), 1);
    map.insert("Y".to_string(), 2);
    map.insert("Z".to_string(), 3);

    let mut total = 0;
    let mut total2 = 0;

    for k in &res {

        let opponent_val = *map.get(&k.0).unwrap();
        let your_val = *map.get(&k.1).unwrap();
    

        if opponent_val == your_val {

            total += 3 + your_val;

        }

        // Rock | Paper
        else if k.0 == "A" && k.1 == "Y" {

            total += 6 + your_val;

        }

        // Paper | Scissors
        else if k.0 == "B" && k.1 == "Z" {

            total += 6 + your_val;

        }

        // Paper | Rock
        else if k.0 == "B" && k.1 == "X" {

            total += your_val;

        }

        // Scissors | Paper
        else if k.0 == "C" && k.1 == "B" {

            total += your_val;

        }

        // Rock | Paper
        else if k.0 == "A" && k.1 == "Y" {

            total += 6 + your_val;

        }

        // Rock | Scissors
        else if k.0 == "A" && k.1 == "Z" {

            total += your_val;

        }

        // Scissors | Rock
        else if k.0 == "C" && k.1 == "A" {

            total += 6 + your_val;

        }

        // Scissors | Paper
        else if k.0 == "C" && k.1 == "Y" {

            total += your_val;

        }

        // Scissors | Rock
        else if k.0 == "C" && k.1 == "X" {

            total += 6 + your_val;

        }

    }

    for k in &res {

        let opponent_val = *map.get(&k.0).unwrap();
        let your_val = *map.get(&k.1).unwrap();

        // Lose
        if k.1 == "X" {

            if k.0 == "A" { total2 += map.get("Z").unwrap(); }
            else if k.0 == "B" { total2 += map.get("X").unwrap(); }
            else if k.0 == "C" { total2 += map.get("Y").unwrap(); }  

        }

        // Draw 
        else if k.1 == "Y" {

            total2 += 3 + map.get(&k.0).unwrap()

        }

        // Win
        else if k.1 == "Z" {

            if k.0 == "A" { total2 += 6 + map.get("Y").unwrap(); }
            else if k.0 == "B" { total2 += 6 + map.get("Z").unwrap(); }
            else if k.0 == "C" { total2 += 6 + map.get("X").unwrap(); }  

        }

    }

    return (total, total2);

}
