use std::{fs, collections::{VecDeque, HashSet}, hash::Hash, cmp, string};

static cycles: [i32; 6] = [20, 60, 100, 140, 180, 220];

#[derive(Debug)]
struct CPU {

    X: i32,
    cycles: i32,

}

pub fn run() -> (String, String) {

    let input = fs::read_to_string("src/day10/input.txt")
    .expect("Should have been able to read the file");

    let p1 = part1(&input);
    let p2 = part2(&input);

    return (p1, p2);

}

fn part2(input: &str) -> String {

    let mut crt = vec![vec!["."; 40]; 6];

    let mut crt_pos = 0;

    let mut cpu = CPU{X: (1), cycles: (0)};

    let mut cycle = |cpu: &mut CPU, crt_pos: &mut i32| { 

        cpu.cycles += 1;
        
        let x = cpu.X;
        let y = cpu.cycles / 40;

        for x1 in (-1..=1).collect::<Vec<i32>>() {

            if (x + x1) >= 0 && (x + x1) < crt[0].len() as i32 {

                if *crt_pos == (x + x1) {

                    crt[y as usize][(x+x1) as usize] = "#";

                }

            }

        }

        *crt_pos = (*crt_pos + 1) % 40;

    };


    for line in input.lines() {

        // noop
        if line.len() == 4 {

            cycle(&mut cpu, &mut crt_pos);

        }
        // addx
        else {

            let mut split = line.split(" ");

            let num = split.nth(1).unwrap().parse::<i32>().unwrap();

            cycle(&mut cpu, &mut crt_pos);
            cycle(&mut cpu, &mut crt_pos);

            cpu.X += num;

        }


    }

    let mut res = String::new();

    res += "\n";

    for row in crt {

        for col in row {
            res += col;

        }

        res += "\n";

    }

    res

}

fn part1(input: &str) -> String {

    let mut cpu = CPU{X: (1), cycles: (0)};

    let mut res: i32 = 0;

    let cycle = |cpu: &mut CPU| -> i32 { 
        cpu.cycles += 1;
        let mut res = 0;
        if cycles.contains(&cpu.cycles) {
            //dbg!(&cpu);
            res += cpu.cycles * cpu.X;

        }
        res
    };

    for line in input.lines() {

        // noop
        if line.len() == 4 {

            res += cycle(&mut cpu);

        }
        // addx
        else {

            let mut split = line.split(" ");

            let num = split.nth(1).unwrap().parse::<i32>().unwrap();

            res += cycle(&mut cpu);
            res += cycle(&mut cpu);

            cpu.X += num;

        }


    }
    res.to_string()

}