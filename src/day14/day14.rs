use std::{fs, cmp};

pub fn run() -> (u32, u32) {

    let input = fs::read_to_string("src/day14/input.txt")
    .expect("Should have been able to read the file");

    let p1 = part1(&input);
    let p2 = part2(&input);

    return (p1, p2);

}


fn print_array(array: &Vec<Vec<&str>>) {

    for row in 0..array.len() {

        for col in 0..array[0].len() {

            print!("{}", array[row][col]);

        }

        println!();

    }

}

fn part1(input: &str) -> u32 {

    let mut min_x = i32::MAX;
    let mut max_x = 0;

    let mut max_y = 0;

    for line in input.lines() {

        for coord in line.split(" -> ") {

            let mut split = coord.split(",");
            let x = split.next().unwrap().parse::<i32>().unwrap();
            let y = split.next().unwrap().parse::<i32>().unwrap();

            min_x = cmp::min(min_x, x);
            max_x = cmp::max(max_x, x);
            max_y = cmp::max(max_y, y);

        }

    }

    let x_len = max_x - min_x + 1;

    let mut array = vec![vec!["."; x_len as usize]; (max_y + 1) as usize];

    //print_array(&array);

    for line in input.lines() {

        for chunk in line.split(" -> ").collect::<Vec<_>>().windows(2) {
            
            let coord1 = chunk[0];
            let coord2 = chunk[1];
            
            let mut split1 = coord1.split(",");
            let x1 = split1.next().unwrap().parse::<i32>().unwrap();
            let y1 = split1.next().unwrap().parse::<i32>().unwrap();

            let mut split2 = coord2.split(",");
            let x2 = split2.next().unwrap().parse::<i32>().unwrap();
            let y2 = split2.next().unwrap().parse::<i32>().unwrap();

            //println!("({x1},{y1}) to ({x2}, {y2})");

            if x1 == x2 {

                for y in cmp::min(y1, y2)..=cmp::max(y1, y2) {

                    array[y as usize][(x1 - min_x) as usize] = "#";

                }

            }
            else if y1 == y2 {

                for x in cmp::min(x1, x2)..=cmp::max(x1, x2) {

                    array[y1 as usize][(x - min_x) as usize] = "#";

                }

            }
            else {

                panic!("Same coord");

            }

        }

    }

    //println!();
    //print_array(&array);

    let check = |array: &Vec<Vec<&str>>, pos: &(i32, i32)| -> bool {

        let i = pos.0;
        let j = pos.1;

        if i < 0 || i >= array[0].len() as i32{
            return false;
        }

        if j > max_y {
            return false;
        }

        // if array[(j + 1) as usize][i as usize] == "#" && array[(j + 1) as usize][(i - 1) as usize] == "#" && array[(j + 1) as usize][(i + 1) as usize] == "#" {
        //     return false;
        // }

        if (j+1) <= max_y && array[(j + 1) as usize][i as usize] != "#" {
            return true;
        }
        if (j+1) <= max_y && (i-1) >= 0 && array[(j + 1) as usize][(i - 1) as usize] != "#" {
            return true;
        }
        if (j+1) <= max_y && (i+1) < array[0].len() as i32 && array[(j + 1) as usize][(i + 1) as usize] != "#" {
            return true;
        }

        false

    };

    let mut sand = 0;

    while true {

        let mut sand_pos = (500 - min_x, 0);

        while check(&array, &sand_pos) {

            //println!("{}, {} ",sand_pos.0,sand_pos.1);

            if array[(sand_pos.1 + 1) as usize][sand_pos.0 as usize] != "#" {

                sand_pos.1 += 1;

            }

            else if array[(sand_pos.1 + 1) as usize][(sand_pos.0 - 1) as usize] != "#" {

                sand_pos.0 -= 1;
                sand_pos.1 += 1;

            }

            else if array[(sand_pos.1 + 1) as usize][(sand_pos.0 + 1) as usize] != "#" {

                sand_pos.0 += 1;
                sand_pos.1 += 1;

            }

        }

        if sand_pos.1 as i32 > max_y - 1{
            //print_array(&array);
            return sand;
        }

        if sand_pos.0 < 1 || sand_pos.0 > array[0].len() as i32 - 1{
            //print_array(&array);
            return sand;
        }

        array[sand_pos.1 as usize][sand_pos.0 as usize] = "#";

        sand += 1;

    }

    //print_array(&array);

    0

}

fn part2(input: &str) -> u32 {

    let mut min_x = i32::MAX;
    let mut max_x = 0;

    let mut max_y = 0;

    for line in input.lines() {

        for coord in line.split(" -> ") {

            let mut split = coord.split(",");
            let x = split.next().unwrap().parse::<i32>().unwrap();
            let y = split.next().unwrap().parse::<i32>().unwrap();

            min_x = cmp::min(min_x, x);
            max_x = cmp::max(max_x, x);
            max_y = cmp::max(max_y, y);

        }

    }


    max_y += 2;
    //min_x -= max_y - 6;


        
    //let x_len = (max_x - min_x + 1) + max_y + 3;

    let x_len = 500 + max_y + 2;

    let mut array = vec![vec!["."; x_len as usize]; (max_y + 1) as usize];

    //print_array(&array);

    //println!("{max_y}, {x_len}");

    for line in input.lines() {

        for chunk in line.split(" -> ").collect::<Vec<_>>().windows(2) {
            
            let coord1 = chunk[0];
            let coord2 = chunk[1];
            
            let mut split1 = coord1.split(",");
            let x1 = split1.next().unwrap().parse::<i32>().unwrap();
            let y1 = split1.next().unwrap().parse::<i32>().unwrap();

            let mut split2 = coord2.split(",");
            let x2 = split2.next().unwrap().parse::<i32>().unwrap();
            let y2 = split2.next().unwrap().parse::<i32>().unwrap();

            //println!("({x1},{y1}) to ({x2}, {y2})");

            if x1 == x2 {

                for y in cmp::min(y1, y2)..=cmp::max(y1, y2) {

                    array[y as usize][(x1) as usize] = "#";

                }

            }
            else if y1 == y2 {

                for x in cmp::min(x1, x2)..=cmp::max(x1, x2) {

                    array[y1 as usize][(x) as usize] = "#";

                }

            }
            else {

                panic!("Same coord");

            }

        }

    }


    for x in 0..array[0].len() {

        array[(max_y) as usize][x] = "#";

    }
    //println!();
    //print_array(&array);

    let check = |array: &Vec<Vec<&str>>, pos: &(i32, i32)| -> bool {

        let i = pos.0;
        let j = pos.1;

        if i < 0 || i >= array[0].len() as i32{
            return false;
        }

        if j > max_y {
            return false;
        }

        // if array[(j + 1) as usize][i as usize] == "#" && array[(j + 1) as usize][(i - 1) as usize] == "#" && array[(j + 1) as usize][(i + 1) as usize] == "#" {
        //     return false;
        // }

        if (j+1) <= max_y && array[(j + 1) as usize][i as usize] != "#" {
            return true;
        }
        if (j+1) <= max_y && (i-1) >= 0 && array[(j + 1) as usize][(i - 1) as usize] != "#" {
            return true;
        }
        if (j+1) <= max_y && (i+1) < array[0].len() as i32 && array[(j + 1) as usize][(i + 1) as usize] != "#" {
            return true;
        }

        false

    };

    let mut sand = 0;

    while true {

        let mut sand_pos = (500, 0);

        while check(&array, &sand_pos) {

            //println!("{} || {}, {} ", sand, sand_pos.0,sand_pos.1);

            if array[(sand_pos.1 + 1) as usize][sand_pos.0 as usize] != "#" {

                sand_pos.1 += 1;

            }

            else if array[(sand_pos.1 + 1) as usize][(sand_pos.0 - 1) as usize] != "#" {

                sand_pos.0 -= 1;
                sand_pos.1 += 1;

            }

            else if array[(sand_pos.1 + 1) as usize][(sand_pos.0 + 1) as usize] != "#" {

                sand_pos.0 += 1;
                sand_pos.1 += 1;

            }

        }

        if sand_pos == (500, 0) {
            //print_array(&array);
            return 1 + sand;
        }

        if sand_pos.1 as i32 > max_y - 1{
            //print_array(&array);
            return sand;
        }

        if sand_pos.0 < 1 || sand_pos.0 > array[0].len() as i32 - 1{
            //print_array(&array);
            return sand;
        }

        array[sand_pos.1 as usize][sand_pos.0 as usize] = "#";

        sand += 1;

    }

    //print_array(&array);

    0

}