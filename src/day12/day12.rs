// use std::{cmp, fs};

// pub fn run() -> (u32, u32) {

//     let input = fs::read_to_string("src/day12/test.txt")
//     .expect("Should have been able to read the file");

//     let p1 = part1(&input);
//     let p2 = part2(&input);

//     return (p1, p2);

// }

// fn bfs(graph: &Vec<Vec<char>>) {

//     let mut queue = Queue();

// }

// fn part1(input: &str) -> u32 {

//     let mut graph: Vec<Vec<char>> = Vec::new();

//     let mut startPos: (usize, usize) = (0, 0);

//     for (i, line) in input.lines().enumerate() {

//         let mut row: Vec<char> = Vec::new();

//         for (j, c) in line.chars().enumerate() {

//             row.push(c);

//             if c == 'S' {

//                 startPos = (j, i);

//             }

//         }

//         graph.push(row);

//     }

//     return dfs(&graph, startPos.0, startPos.1, startPos.1) as u32;

// }

// fn part2(input: &str) -> u32 {0}