mod day1;
mod day2;

fn main() {
    let d1 = day1::day1::run();
    println!("Day1 : {}, {}", d1.0, d1.1);
    
    let d2 = day2::day2::run();
    println!("Day2 : {}, {}", d2.0, d2.1);
}
