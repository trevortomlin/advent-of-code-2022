use std::fs;

pub fn run() -> (u32, u32) {

    let input = fs::read_to_string("src/day4/input.txt")
    .expect("Should have been able to read the file");

    let p1 = part1(&input);
    let p2 = part2(&input);

    return (p1, p2);

}

fn part1(input: &str) -> u32 {

    input.lines()
         .filter(|s| {
            let mut split = s.split(",");

            let h1 = split.next().unwrap();
            let h2 = split.next().unwrap();

            let mut i1 = h1.split("-");
            let mut i2 = h2.split("-");

            let s1 = i1.next().unwrap().parse::<i32>().unwrap();
            let e1 = i1.next().unwrap().parse::<i32>().unwrap();

            let s2 = i2.next().unwrap().parse::<i32>().unwrap();
            let e2 = i2.next().unwrap().parse::<i32>().unwrap();

            (s1 <= s2 && e1 >= e2) || (s2 <= s1 && e2 >= e1)

         })
         .count() as u32

}

fn part2(input: &str) -> u32 {

    input.lines()
         .filter(|s| {
            let mut split = s.split(",");

            let h1 = split.next().unwrap();
            let h2 = split.next().unwrap();

            let mut i1 = h1.split("-");
            let mut i2 = h2.split("-");

            let s1 = i1.next().unwrap().parse::<i32>().unwrap();
            let e1 = i1.next().unwrap().parse::<i32>().unwrap();

            let s2 = i2.next().unwrap().parse::<i32>().unwrap();
            let e2 = i2.next().unwrap().parse::<i32>().unwrap();

            !((e1 < s2) || (s1 > e2))

         })
         .count() as u32

}