use std::{fs, collections::{VecDeque, HashSet}, hash::Hash, cmp};

#[derive(Copy, Clone)]
struct Segment {

    x: i32,
    y: i32,
    s_type: SegmentType,

}

#[derive(Copy, Clone, PartialEq)]
enum SegmentType {
    Tail,
    Head,
    None,
}

pub fn run() -> (u32, u32) {

    let input = fs::read_to_string("src/day9/input.txt")
    .expect("Should have been able to read the file");

    let p1 = part1(&input);
    let p2 = part2(&input);

    return (p1, p2);

}

fn part1(input: &str) -> u32 {

    let mut steps: Vec<(&str, i32)> = Vec::new();

    for line in input.lines() {

        let mut split = line.split_whitespace();

        let dir = split.next().unwrap();
        let amt = split.next().unwrap();
        
        let amt = amt.parse::<i32>().unwrap();

        steps.push((dir, amt));

    }

    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);

    let mut visited = HashSet::new();

    for step in steps {

        let dir = step.0;
        let amt = step.1;

        for _ in 0..amt {

            match dir {

                "U" => head.1 += 1,
                "D" => head.1 -= 1,
                "L" => head.0 -= 1,
                "R" => head.0 += 1,
                _ => panic!("Wrong Direction")

            }

            let mut subx: i32 = head.0 - tail.0;
            let mut suby: i32 = head.1 - tail.1;

            if subx < 0 {
                subx *= -1;
            }
            if suby < 0 {
                suby *= -1;
            }

            let chev_d = cmp::max(subx, suby);

            if chev_d > 1 {

                let diffx = head.0 - tail.0;
                let diffy = head.1 - tail.1;

                // up or down
                if diffx == 0 && (diffy == 2 || diffy == -2) {

                    tail.1 += diffy / 2;

                }

                // left or right
                else if diffy == 0 && (diffx == 2 || diffx == -2) {

                    tail.0 += diffx / 2;

                }

                else {

                    if diffx > 0 {

                        tail.0 += 1

                    } 

                    else if diffx < 0 {

                        tail.0 -= 1

                    }

                    if diffy > 0 {

                        tail.1 += 1

                    } 

                    else if diffy < 0 {

                        tail.1 -= 1

                    }

                }

            }

            visited.insert((tail.0, tail.1));

        }

    }

    visited.len() as u32

}

fn part2(input: &str) -> u32 {

    let mut steps: Vec<(&str, i32)> = Vec::new();

    for line in input.lines() {

        let mut split = line.split_whitespace();

        let dir = split.next().unwrap();
        let amt = split.next().unwrap();
        
        let amt = amt.parse::<i32>().unwrap();

        steps.push((dir, amt));

    }

    let mut segments: Vec<Segment> = Vec::new();

    segments.push(Segment { x: (0), y: (0), s_type: (SegmentType::Head) });

    for _ in 1..9 {

        segments.push(Segment { x: (0), y: (0), s_type: (SegmentType::None) });

    }

    segments.push(Segment { x: (0), y: (0), s_type: (SegmentType::Tail) });

    let mut visited = HashSet::new();

    for step in steps {

        let dir = step.0;
        let amt = step.1;

        println!("{},{}", segments[0].x, segments[0].y);
        println!("{},{}", segments[9].x, segments[9].y);

        for _ in 0..amt {

            for i in 0..segments.len()-1 {

                //let mut head = segments[i];
                //let mut tail = segments[i+1];

                if segments[i].s_type == SegmentType::Head {

                    match dir {

                        "U" => segments[i].y += 1,
                        "D" => segments[i].y -= 1,
                        "L" => segments[i].x -= 1,
                        "R" => segments[i].x += 1,
                        _ => panic!("Wrong Direction")

                    }

                }

                let mut subx: i32 = segments[i].x - segments[i+1].x;
                let mut suby: i32 = segments[i].y - segments[i+1].y;

                if subx < 0 {
                    subx *= -1;
                }
                if suby < 0 {
                    suby *= -1;
                }

                let chev_d = cmp::max(subx, suby);

                if chev_d > 1 {

                    let diffx = segments[i].x - segments[i+1].x;
                    let diffy = segments[i].y - segments[i+1].y;

                    // up or down
                    if diffx == 0 && (diffy == 2 || diffy == -2) {

                        segments[i+1].y += diffy / 2;

                    }

                    // left or right
                    else if diffy == 0 && (diffx == 2 || diffx == -2) {

                        segments[i+1].x += diffx / 2;

                    }

                    else {

                        if diffx > 0 {

                            segments[i+1].x += 1

                        } 

                        else if diffx < 0 {

                            segments[i+1].x -= 1

                        }

                        if diffy > 0 {

                            segments[i+1].y += 1

                        } 

                        else if diffy < 0 {

                            segments[i+1].y -= 1

                        }

                    }

                }

                if segments[i+1].s_type == SegmentType::Tail {
                    visited.insert((segments[i+1].x, segments[i+1].y));
                }

            }

        }

    }

    visited.len() as u32

}