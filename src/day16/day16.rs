// use std::{fs, collections::{HashMap, HashSet}, cmp};

// pub fn run() -> (u32, u32) {

//     let input = fs::read_to_string("src/day16/test.txt")
//     .expect("Should have been able to read the file");

//     let p1 = part1(&input);
//     let p2 = part2(&input);

//     return (p1, p2);

// }

// #[derive(Debug)]
// struct Valve {

//     name: String,
//     flow_rate: u32,
//     tunnels: Vec<String>,

// }

// fn dfs(map: &HashMap<String, Valve>, set: &HashSet<String>, name: String, minutes: i32) -> i32 {

//     if minutes <= 0 {
//         return 0;
//     }

//     let mut max = 0;
//     let valve = map.get(&name).unwrap();

//     //println!("{name}, {minutes}");
//     let pressure = valve.flow_rate as i32 * (minutes - 1);


//     for other in &valve.tunnels {

//         if !set.contains(&name) && pressure != 0{
//             let mut cur_set = set.clone();
//             cur_set.insert(name.clone());
//             max = cmp::max(max, pressure + dfs(map, &cur_set, other.clone(), minutes-2));
//         }

//         max = cmp::max(max, dfs(map, set, other.clone(), minutes-1));

//     }

//     max
// }

// //fn bfs()

// // fn dfs(map: &HashMap<String, Valve>, set: &HashSet<String>, name: String, minutes: u32, cache: &mut HashMap<(u32, String), u32>) -> u32 {

// //     if minutes <= 0 {
// //         return 0;
// //     }

// //     // if cache.contains_key(&(minutes, name.clone())) {
// //     //     return *cache.get(&(minutes, name)).unwrap();
// //     // }

// //     // if set.contains(&name) {
// //     //     return 0;
// //     // }


// //     let valve = map.get(&name).unwrap();

// //     let mut max = 0;

// //     let pressure = valve.flow_rate * (minutes - 1);

    
// //     println!("{name}, {minutes} , {pressure}");

// //     let mut cur_set = set.clone();
// //     cur_set.insert(name.clone());

// //     //println!("{:?}", cur_set);

// //     for other in &valve.tunnels {

// //         if !set.contains(&name) && pressure != 0 {

// //             max = cmp::max(max, pressure + dfs(map, &cur_set, other.clone(), minutes-2, cache));

// //         }

// //         max = cmp::max(max, dfs(map, set, other.clone(), minutes-1, cache));

// //     }


// //     // for other in &valve.tunnels {

// //     //     max = cmp::max(max, dfs(map, set, other.clone(), minutes+1));
// //     //     // if !set.contains(&name) {
// //     //     //     max = cmp::max(max, valve.flow_rate * (30 - minutes) + dfs(map, set, other.clone(), minutes+2));
// //     //     // }


// //     // }

// //     // if !set.contains(&name) && valve.flow_rate > 0 {
// //     //     set.insert(name.clone());
// //     //     max = cmp::max(max, valve.flow_rate * (30 - minutes - 1) + dfs(map, set, name.clone(), minutes+1));
// //     //     set.remove(&name);
// //     // }
    
// //     //cache.insert((minutes, name), max);

// //     max
// // }

// fn part1(input: &str) -> u32 {

//     //let mut valves = Vec::new();

//     let mut name_to_valve: HashMap<String, Valve> = HashMap::new();
//     let visited: HashSet<String> = HashSet::new();
//     let mut cache: HashMap<(u32, String), u32> = HashMap::new();

//     for line in input.lines() {

//         let mut valve = Valve{ name: "".to_string(), flow_rate: 0, tunnels: Vec::new() };

//         let mut split = line.split(" ");

//         let name = split.nth(1).unwrap();

//         let flow_rate = split.nth(2).unwrap().replace("rate=", "").replace(";", "").parse::<u32>().unwrap();

//         valve.name = name.to_string();
//         valve.flow_rate = flow_rate;

//         for v in split.skip(4) {
//             valve.tunnels.push(v.to_string().replace(",", ""));
//         }

//        //println!("{:?}", valve);

//        //valves.push(valve);
//        name_to_valve.insert(name.to_string(), valve);

//     }

//     //dfs(&name_to_valve, &visited, "AA".to_string(), 30, &mut cache)

//     println!("{}", dfs(&name_to_valve, &visited, "AA".to_string(), 30,));

//     0

// }
// fn part2(input: &str) -> u32 {0}