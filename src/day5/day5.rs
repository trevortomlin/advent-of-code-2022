use std::{fs, collections::VecDeque};

pub fn run() -> (String, String) {

    let input = fs::read_to_string("src/day5/input.txt")
    .expect("Should have been able to read the file");

    let p1 = part1(&input);
    let p2 = part2(&input);

    return (p1, p2);

}

fn part1(input: &str) -> String {

    let mut stacks: Vec<VecDeque<char>> = Vec::new();

    let mut parseStack: bool = true;

    for line in input.lines() {

        while parseStack && stacks.len() < line.len() / 3 {

            let stack: VecDeque<char> = VecDeque::new();
            stacks.push(stack);

        }

        if line.len() == 0 || line[0..=1] == " 1".to_string() {
            parseStack = false;
            continue;
        }

        else if parseStack {

            for (i, char) in line.chars().enumerate() {

                let stackIndex = (i  / 4);

                if char != ' ' && char != '[' && char != ']'{
                    println!("i: {i}, Stack: {stackIndex}, Char: {char}");

                    stacks[stackIndex].push_back(char);

                }

            }

        }

        else {

            let num = line.split(" ").nth(1).unwrap().parse::<usize>().unwrap();
            let from = line.split(" ").nth(3).unwrap().parse::<usize>().unwrap();
            let to = line.split(" ").nth(5).unwrap().parse::<usize>().unwrap();

            for _ in 0..num {

                let first = stacks[from-1].pop_front().unwrap();
                stacks[to-1].push_front(first);

            }


        }

    }

    let mut res = "".to_string();

    for stack in stacks {
        if stack.len() > 0 {
            res.push(*stack.get(0).unwrap());
        }

    }

    res.to_string()

}

fn part2(input: &str) -> String {

    let mut stacks: Vec<VecDeque<char>> = Vec::new();

    let mut parseStack: bool = true;

    for line in input.lines() {

        while parseStack && stacks.len() < line.len() / 3 {

            let stack: VecDeque<char> = VecDeque::new();
            stacks.push(stack);

        }

        if line.len() == 0 || line[0..=1] == " 1".to_string() {
            parseStack = false;
            continue;
        }

        else if parseStack {

            for (i, char) in line.chars().enumerate() {

                let stackIndex = (i  / 4);

                if char != ' ' && char != '[' && char != ']'{
                    println!("i: {i}, Stack: {stackIndex}, Char: {char}");

                    stacks[stackIndex].push_back(char);

                }

            }

        }

        else {

            let num = line.split(" ").nth(1).unwrap().parse::<usize>().unwrap();
            let from = line.split(" ").nth(3).unwrap().parse::<usize>().unwrap();
            let to = line.split(" ").nth(5).unwrap().parse::<usize>().unwrap();

            let chunk = stacks[from-1].drain(0..num).collect::<Vec<_>>();

            for i in chunk.iter().rev() {

                stacks[to-1].push_front(*i);

            }


        }

    }

    let mut res = "".to_string();

    for stack in stacks {
        if stack.len() > 0 {
            res.push(*stack.get(0).unwrap());
        }

    }

    res.to_string()

}