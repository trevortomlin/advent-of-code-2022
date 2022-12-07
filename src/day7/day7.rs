// use std::{fs, fmt};
// use std::cell::RefCell;
// use std::rc::Rc;

// #[derive(PartialEq)]
// struct TreeNode {
//   pub value: Option<File>,
//   pub children: Vec<Rc<RefCell<TreeNode>>>,
//   pub parent: Option<Rc<RefCell<TreeNode>>>,
// }

// impl TreeNode {
//   pub fn new() -> TreeNode {
//     return TreeNode {
//       value: None,
//       children: vec![],
//       parent: None,
//     };
//   }

//   pub fn add_child(&mut self, new_node: Rc<RefCell<TreeNode>>) {
//     self.children.push(new_node);
//   }

//   pub fn print(&self) -> String {
//     if let Some(value) = self.value {
//       return value.to_string();
//     } else {
//       return String::from("[")
//         + &self
//           .children
//           .iter()
//           .map(|tn| tn.borrow().print())
//           .collect::<Vec<String>>()
//           .join(",")
//         + "]";
//     }
//   }
// }

// #[derive(PartialEq)]
// enum File {

//     File((String, u32)),
//     Dir(String),

// }

// impl fmt::Display for File {

//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             File::File((title, size)) => write!(f, "{title}, {size}"),
//             File::Dir(title) => write!(f, "{title}"),
//         }
//      }

// }

// pub fn run() -> (u32, u32) {

//     let input = fs::read_to_string("src/day7/test.txt")
//     .expect("Should have been able to read the file");

//     let p1 = part1(&input);
//     let p2 = part2(&input);

//     return (p1, p2);

// }

// fn part1(input: &str) -> u32 {

//     let mut current_dir = Rc::new(RefCell::new(TreeNode::new()));

//     let root = Rc::new(RefCell::new(TreeNode::new()));
//     root.borrow_mut().value = Some(File::Dir("/".to_string()));


//     for line in input.lines() {

//         let command: Vec<&str> = line.split(" ").collect();

//         let first = *command.iter().next().unwrap();
//         let second = *command.iter().nth(1).unwrap();

//         match first {

//             "$" => {
                
//                 match second {

//                     "cd" => {
                        
//                         for node in &current_dir.borrow().children {

//                             match &node.borrow().value {

//                                 Some(File::File((_, _))) => {}
//                                 Some(File::Dir(title)) => {

//                                     current_dir = *node;

//                                 }
//                                 None => {}

//                             }

//                         } 

//                     },
//                     "ls" => {}
//                     _ => {}

//                 }

//             },
//             "dir" => {println!("Dir")}
//             _ => {println!("File")}

//         };

//     }
    
//     0

// }

// fn part2(input: &str) -> u32 {

//     0      

// }