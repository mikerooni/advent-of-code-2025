use std::fs;

pub fn print_program_header(day: u8, problem_name: &str) {
    println!("+-----------------------------------------------+");
    println!("|              ADVENT OF CODE 2025              |");
    println!("+-----------------------------------------------+");
    println!();
    println!("DAY {day:02}: {problem_name}");
    println!();
}

pub fn read_data(day: u8) -> String {
    let data = fs::read(format!("./data/day{}.txt", day)).expect("Cannot open the input file.");

    String::from_utf8(data).expect("Input file cannot be read as UTF-8")
}
