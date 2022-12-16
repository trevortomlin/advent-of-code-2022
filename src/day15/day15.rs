use std::{fs, cmp, collections::HashSet};

#[derive(Debug)]
struct Data {

    sensor: (i32, i32),
    beacon: (i32, i32),

}

const ROW: i32 = 2000000;
const C_MAX: i32 = 4000000;
const C_MIN: i32 = 0;
//const ROW: i32 = 10;

pub fn run() -> (u128, u128) {

    let input = fs::read_to_string("src/day15/input.txt")
    .expect("Should have been able to read the file");

    let p1 = part1(&input);
    let p2 = part2(&input);

    return (p1, p2);

}

fn manhatten_distance(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    
    let mut x = p1.0 - p2.0;
    let mut y = p1.1 - p2.1;

    if x < 0 {
        x *= -1;
    }
    if y < 0 {
        y *= -1;
    }

    return x + y;
}

fn part1(input: &str) -> u128 {

    let mut coords: Vec<Data> = Vec::new();

    for line in input.lines() {

        let mut datum = Data{ sensor: (0, 0), beacon: (0, 0) };

        let mut x = String::new();
        let mut y = String::new();

        let mut sensor = true;

        let chars = line.chars().collect::<Vec<_>>();

        let mut i = 0;
        while i < chars.len() {

            let mut char = chars[i];

            //println!("{char}");

            if char == 'x' {

                i += 2;
                char = chars[i];

                while char != ',' {

                    x.push(char);
                    i += 1;

                    if i >= chars.len() {break;}

                    char = chars[i];

                }

                if sensor {
                    //println!("{x}");
                    datum.sensor.0 = x.parse::<i32>().unwrap();
                }
                else {
                    datum.beacon.0 = x.parse::<i32>().unwrap();
                }

                x.clear();



            }

            if char == 'y' {

                i += 2;
                char = chars[i];

                while char != ':' {
                    //println!("{char}");
                    y.push(char);
                    i += 1;

                    if i >= chars.len() {break;}

                    char = chars[i];

                }

                if sensor {
                    datum.sensor.1 = y.parse::<i32>().unwrap();
                }
                else {
                    datum.beacon.1 = y.parse::<i32>().unwrap();
                }

                y.clear();

            }

            if char == ':' {
                sensor = false;
            }

            i+=1;

        }

        coords.push(datum);

    }

    let mut set: HashSet<i32> = HashSet::new();

    for coord in coords {

        let d = manhatten_distance(coord.sensor, coord.beacon);

        let mut dy = coord.sensor.1 - ROW;

        if dy < 0 {
            dy *= -1;
        }

        let dx = d - dy;

        for x in coord.sensor.0 - dx..coord.sensor.0 + dx {
            set.insert(x);
        }

       

    }

    set.len() as u128

}

fn part2(input: &str) -> u128 {

    let mut coords: Vec<Data> = Vec::new();

    for line in input.lines() {

        let mut datum = Data{ sensor: (0, 0), beacon: (0, 0) };

        let mut x = String::new();
        let mut y = String::new();

        let mut sensor = true;

        let chars = line.chars().collect::<Vec<_>>();

        let mut i = 0;
        while i < chars.len() {

            let mut char = chars[i];

            //println!("{char}");

            if char == 'x' {

                i += 2;
                char = chars[i];

                while char != ',' {

                    x.push(char);
                    i += 1;

                    if i >= chars.len() {break;}

                    char = chars[i];

                }

                if sensor {
                    //println!("{x}");
                    datum.sensor.0 = x.parse::<i32>().unwrap();
                }
                else {
                    datum.beacon.0 = x.parse::<i32>().unwrap();
                }

                x.clear();



            }

            if char == 'y' {

                i += 2;
                char = chars[i];

                while char != ':' {
                    //println!("{char}");
                    y.push(char);
                    i += 1;

                    if i >= chars.len() {break;}

                    char = chars[i];

                }

                if sensor {
                    datum.sensor.1 = y.parse::<i32>().unwrap();
                }
                else {
                    datum.beacon.1 = y.parse::<i32>().unwrap();
                }

                y.clear();

            }

            if char == ':' {
                sensor = false;
            }

            i+=1;

        }

        coords.push(datum);

    }

    let mut pos = Vec::new();
    let mut neg = Vec::new();

    for coord in &coords {

        let d = manhatten_distance(coord.sensor, coord.beacon);

        neg.push(coord.sensor.0 + coord.sensor.1 - d);
        neg.push(coord.sensor.0 + coord.sensor.1 + d);

        pos.push(coord.sensor.0 - coord.sensor.1 - d);
        pos.push(coord.sensor.0 - coord.sensor.1 + d);

    }

    let mut pos_line = 0;
    let mut neg_line = 0;

    for i in 0..2*&coords.len() {

        for j in i+1..2*&coords.len() {

            let a = pos[i];
            let b = pos[j];

            if a - b == 2 || a - b == -2 {
                pos_line = cmp::min(a, b) + 1;
            }

            let a = neg[i];
            let b = neg[j];

            if a - b == 2 || a - b == -2 {
                neg_line = cmp::min(a, b) + 1;
            }

        }

    }

    let x = (pos_line + neg_line) / 2;
    let y = (neg_line - pos_line) / 2;

    return x as u128 * 4_000_000 as u128 + y as u128;

}