mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;

enum RunPointer {

    Num(fn() -> (u32, u32)),
    Str(fn() -> (String, String)),
    Ext(&'static str)

}

static RUNS: [RunPointer; 10] =      [
                                    RunPointer::Num(day1::day1::run),
                                    RunPointer::Num(day2::day2::run),
                                    RunPointer::Num(day3::day3::run),
                                    RunPointer::Num(day4::day4::run),
                                    RunPointer::Str(day5::day5::run),
                                    RunPointer::Num(day6::day6::run),
                                    RunPointer::Ext("Python: 1555642, 5974547"),
                                    RunPointer::Num(day8::day8::run),
                                    RunPointer::Num(day9::day9::run),
                                    RunPointer::Str(day10::day10::run),
                                   ];

fn main() {

    for (i, run) in RUNS.iter().enumerate() {

        match run {

            RunPointer::Num(x) => {
                let d = x(); 
                println!("Day{} : {}, {}", i+1, d.0, d.1);
                },

            RunPointer::Str(x) => {
                let d = x(); 
                println!("Day{} : {}, {}", i+1, d.0, d.1);
            },
            RunPointer::Ext(title) => {
                println!("Day{} : Completed in {title}", i+1);
            }

        }

    }

}
