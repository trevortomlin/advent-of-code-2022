// use std::{fs, collections::{VecDeque, HashSet}, hash::Hash, cmp, string};

// pub fn run() -> (u32, u32) {

//     let input = fs::read_to_string("src/day11/test.txt")
//     .expect("Should have been able to read the file");

//     let p1 = part1(&input);
//     let p2 = part2(&input);

//     return (p1, p2);

// }

// #[derive(Clone)]
// struct Monkey {

//     items: Vec<i32>,
//     operation: String,
//     op_value: i32,
//     test: i32,
//     trueMonkey: i32,
//     falseMonkey: i32,

// }

// fn part1(input: &str) -> u32 {

//     let mut monkies: Vec<Monkey> = Vec::new();

//     let mut monkey0 = Monkey{ items: vec![93, 98], operation: "*".to_string(), op_value: 17, test: 23, trueMonkey: 5, falseMonkey: 3 };
//     let mut monkey1 = Monkey{ items: vec![95, 72, 98, 82, 86], operation: "+".to_string(), op_value: 5, test: 13, trueMonkey: 7, falseMonkey: 6 };
//     let mut monkey2 = Monkey{ items: vec![85, 62, 82, 86, 70, 65, 83, 76], operation: "+".to_string(), op_value: 8, test: 5, trueMonkey: 3, falseMonkey: 0 };
//     let mut monkey3 = Monkey{ items: vec![86, 70, 71, 56], operation: "+".to_string(), op_value: 1, test: 7, trueMonkey: 4, falseMonkey: 5 };
//     let mut monkey4 = Monkey{ items: vec![77, 71, 86, 52, 81, 67], operation: "+".to_string(), op_value: 4, test: 17, trueMonkey: 1, falseMonkey: 6 };
//     let mut monkey5 = Monkey{ items: vec![89, 87, 60, 78, 54, 77, 98], operation: "*".to_string(), op_value: 7, test: 2, trueMonkey: 1, falseMonkey: 4 };
//     let mut monkey6 = Monkey{ items: vec![69, 65, 63], operation: "+".to_string(), op_value: 6, test: 3, trueMonkey: 7, falseMonkey: 2 };
//     let mut monkey7 = Monkey{ items: vec![89], operation: "*".to_string(), op_value: -1, test: 11, trueMonkey: 0, falseMonkey: 2 };

//     monkies.push(monkey0);
//     monkies.push(monkey1);
//     monkies.push(monkey2);
//     monkies.push(monkey3);
//     monkies.push(monkey4);
//     monkies.push(monkey5);
//     monkies.push(monkey6);
//     monkies.push(monkey7);

//     unsafe {

//         for m_index in 0..monkies.len() {

//             let monkey = &mut monkies[m_index];

//             for i in 0..monkey.items.len() {

//                 match monkey.operation.as_str() {

//                     "*" => {

//                         if monkey.op_value == -1 {

//                             monkey.items[i] *=  monkey.items[i];

//                         }

//                         else {

//                             monkey.items[i] *= monkey.op_value;

//                         }

//                     },
//                     "+" => {

//                         if monkey.op_value == -1 {

//                             monkey.items[i] +=  monkey.items[i];

//                         }

//                         else {

//                             monkey.items[i] += monkey.op_value;

//                         }

//                     },
//                     _ => {},

//                 }

//                 monkey.items[i] /= 3;

//                 if monkey.items[i] % monkey.test == 0 {

//                     monkies[monkey.trueMonkey as usize].items.push(monkey.items[i]);
//                     monkey.items.remove(i);

//                 }

//                 else {



//                 }

//             }



//         }

//     }   


//     0

// }

// fn part2(input: &str) -> u32 {


//     for line in input.lines() {

//     }

//     0

// }