use std::fs;
use std::io::stdin;

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

pub fn read_data_stdin() -> String {
    println!("Write \"####END####\" to end the input.");

    let mut input = String::new();

    loop {
        stdin().read_line( & mut input ).unwrap();
        let input_line= input.lines().last().unwrap();

        if input_line == "####END####" {
            break;
        }
    }

    input
}
