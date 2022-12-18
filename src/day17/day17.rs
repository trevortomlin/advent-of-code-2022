use std::fs;

pub fn run() -> (u32, u32) {

    let input = fs::read_to_string("src/day17/input.txt")
    .expect("Should have been able to read the file");

    let p1 = part1(&input);
    let p2 = part2(&input);

    return (p1, p2);

}

struct Rock {
    points: Vec<(i32, i32)>
}

fn print_chamber(chamber: &Vec<Vec<&str>>) {

    for y in (0..chamber.len()).rev() {

        for x in 0..chamber[0].len() {

            print!("{}", chamber[y][x]);

        }

        println!();

    }


}

fn part1(input: &str) -> u32 {

    let mut horizontal = Rock{ points: vec![(0,0), (1,0), (2,0), (3,0)] };
    let mut plus = Rock{ points: vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)] };
    let mut l = Rock{ points: vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)] };
    let mut vertical = Rock{ points: vec![(0, 0), (0, 1), (0, 2), (0, 3)] };
    let mut square = Rock{ points: vec![(0, 0), (0, 1), (1, 0), (1, 1)] };

    let mut rocks = vec![horizontal, plus, l, vertical, square];

    let mut chamber: Vec<Vec<&str>> = Vec::new();

    let mut idx = 0;
    let chars = input.chars().collect::<Vec<_>>();
    let mut char;

    for i in 0..2022 {

        let mut last_row = 0;

        for y in &chamber {

            if y.iter().any(|&x| x == "#") {

                last_row += 1;
            

            }

        }

        while chamber.len() - last_row <= 7 {

            chamber.push(vec!["."; 7]);

        }

        let rocks_len = &rocks.len();

        let mut rock = Rock{points: rocks[i % rocks_len].points.clone()};

        rock.points.iter_mut().for_each(|(x, y)| {
             *x += 2;
             *y += last_row as i32 + 3;
        });

        let mut falling = true;

        while falling {

            char = chars[idx % chars.len()];

            match char {

                '<' => {

                    if rock.points.iter().all(|&(x, y)| x - 1 >= 0 && chamber[y as usize][(x-1) as usize] != "#") {

                        rock.points.iter_mut().for_each(|(x, y)| *x -= 1);

                    }

                },
                '>' => {

                    if rock.points.iter().all(|&(x, y)| x + 1 < chamber[0].len() as i32 && chamber[y as usize][(x+1) as usize] != "#") {

                        rock.points.iter_mut().for_each(|(x, y)| *x += 1);

                    }

                },
                _ => {}

            }
            
            idx+=1;

            if rock.points.iter().all(|&(x, y)| y - 1 >= 0 && chamber[(y-1) as usize][x as usize] != "#") {

                rock.points.iter_mut().for_each(|(x, y)| *y -= 1);
    
            }
    
            else {
                
                falling = false;
                break;
    
            }

        }

        for (x, y) in &rock.points {
            chamber[*y as usize][*x as usize] = "#";
        }


    }

    let mut last_row = 0;

    for y in &chamber {

        if y.iter().any(|&x| x == "#") {

            last_row += 1;
        

        }

    }

    last_row

}

fn part2(input: &str) -> u32 {
    
    
    0
}