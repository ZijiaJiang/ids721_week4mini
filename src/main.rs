use std::env::args;

use clap::Parser;
use mini4::Date;

#[derive(Parser)]
#[clap(version = "1.0", author = "Zijia Jiang", about = "Days matter")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Zijia Jiang")]
    Difference {
        year1: i32,
        month1: i32,
        day1: i32,

        year2: i32,
        month2: i32,
        day2: i32,
    },
}

fn main() {
    let args: Vec<String> = args().collect();
    let cli = Cli::parse_from(args);
    match cli.command {
        Some(Commands::Difference {
            year1,
            month1,
            day1,
            year2,
            month2,
            day2,
        }) => {
            let first = Date {
                year: year1,
                month: month1,
                day: day1,
            };
            let second = Date {
                year: year2,
                month: month2,
                day: day2,
            };
            println!(
                "It has been {} days since your date. ",
                mini4::date_diff(first, second)
            );
        }
        None => println!("No command"),
    }
}
