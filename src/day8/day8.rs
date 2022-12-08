use std::{fs, collections::{VecDeque, HashSet}, hash::Hash};

pub fn run() -> (u32, u32) {

    let input = fs::read_to_string("src/day8/input.txt")
    .expect("Should have been able to read the file");

    let p1 = part1(&input);
    let p2 = part2(&input);

    return (p1, p2);

}

fn part1(input: &str) -> u32 {

    let mut grid: Vec<Vec<u32>> = Vec::new();

    for line in input.lines() {

        grid.push(line.chars()
                      .map(|x| x.to_digit(10).unwrap())
                      .collect::<Vec<u32>>()
                 );

    }

    let mut res = 0;


    for x in 0..grid.len() {

        for y in 0..grid[0].len() {

            if x == 0 || y == 0 || x == grid.len() - 1 || y == grid[0].len() - 1 {
                res+=1;
                continue;
            }

            let mut l = 0;
            let mut r = 0;
            let mut u = 0;
            let mut d = 0;

            for i in 0..y {

                if grid[x][i] > l {

                    l = grid[x][i];

                }

            }

            for i in y+1..grid[x].len() {

                if grid[x][i] > r {

                    r = grid[x][i];

                }

            }

            for j in 0..x {

                if grid[j][y] > u {

                    u = grid[j][y];

                }

            }

            for j in x+1..grid.len() {

                if grid[j][y] > d {

                    d = grid[j][y];

                }

            }

            //println!("{} || l: {l}, r: {r}, u: {u}, d: {d}", grid[x][y]);

            if l < grid[x][y] || r < grid[x][y] || u < grid[x][y] || d < grid[x][y] {

                res += 1;

            }

        }

    }

    res

}

fn part2(input: &str) -> u32 {

    let mut grid: Vec<Vec<u32>> = Vec::new();

    for line in input.lines() {

        grid.push(line.chars()
                      .map(|x| x.to_digit(10).unwrap())
                      .collect::<Vec<u32>>()
                 );

    }

    let mut res = 0;

    for x in 0..grid.len() {

        for y in 0..grid[0].len() {

            if x == 0 || y == 0 || x == grid.len() - 1 || y == grid[0].len() - 1 {
                continue;
            }

            let mut l = 0;
            let mut r = 0;
            let mut u = 0;
            let mut d = 0;

            for i in (0..y).rev() {

                if grid[x][i] < grid[x][y] {

                    l += 1

                }

                else {

                    l+=1;
                    break;

                }

            }
            for i in y+1..grid[x].len() {

                if grid[x][i] < grid[x][y] {

                    r += 1;

                }

                else {

                    r+=1;
                    break;

                }

            }

            for j in (0..x).rev() {

                if grid[j][y] < grid[x][y] {

                    u += 1;

                }

                else {

                    u+=1;
                    break;

                }

            }

            for j in x+1..grid.len() {

                if grid[j][y] < grid[x][y] {

                    d += 1;

                }

                else {

                    d+=1;
                    break;

                }

            }

            if res < u * l * r * d {

                res = u * l * r * d;

            }

        }

    }

    res  

}