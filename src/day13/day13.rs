// use std::{cmp, fs};

// struct List {

//     children: Option<Vec<Box<List>>>, 

// }

// pub fn run() -> (u32, u32) {

//     let input = fs::read_to_string("src/day13/test.txt")
//     .expect("Should have been able to read the file");

//     let p1 = part1(&input);
//     let p2 = part2(&input);

//     return (p1, p2);

// }

// fn parse_list(list: &str) -> &Box<List> {

//     let mut cur_list = Box::new(List {children: None});
//     let head = &cur_list;



//     return head;

// }

// fn part1(input: &str) -> u32 {

//     let mut lines = input.lines();

//     while let (Some(a), Some(b), Some(_)) = (lines.next(), lines.next(), lines.next()) {

//         println!("{a}");
//         println!("{b}");

//         parse_list(a);

//         break;

//     }

//     0

// }

// fn part2(input: &str) -> u32 {0}