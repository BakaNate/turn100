use std::env;
use::chrono;
use chrono::{DateTime, Local, Datelike};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Error: Missing parameters !");
        return;
    }

    let age: i8 = args[1].parse().unwrap();
    println!("You were born in: {:?}", get_born_year(age));
    println!("You will turn 100 years old in {:?}", get_born_year(age) + 100);
}

pub fn get_born_year(age: i8) -> i32 {
    let date: DateTime<Local> = Local::now();
    date.year() - age as i32
}