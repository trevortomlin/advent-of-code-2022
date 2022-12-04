mod day1;
mod day2;
mod day3;

fn main() {
    let d1 = day1::day1::run();
    println!("Day1 : {}, {}", d1.0, d1.1);
    
    let d2 = day2::day2::run();
    println!("Day2 : {}, {}", d2.0, d2.1);

    let d3 = day3::day3::run();
    println!("Day3 : {}, {}", d3.0, d3.1);
}
