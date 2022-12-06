mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

enum RunPointer {

    Num(fn() -> (u32, u32)),
    Str(fn() -> (String, String)),

}

static runs: [RunPointer; 6] =      [
                                    RunPointer::Num(day1::day1::run),
                                    RunPointer::Num(day2::day2::run),
                                    RunPointer::Num(day3::day3::run),
                                    RunPointer::Num(day4::day4::run),
                                    RunPointer::Str(day5::day5::run),
                                    RunPointer::Num(day6::day6::run),
                                   ];

fn main() {

    for (i, run) in runs.iter().enumerate() {

        match run {

            RunPointer::Num(x) => {
                let d = x(); 
                println!("Day{} : {}, {}", i+1, d.0, d.1);
                },

            RunPointer::Str(x) => {
                let d = x(); 
                println!("Day{} : {}, {}", i+1, d.0, d.1);
            },

        }

    }

}
