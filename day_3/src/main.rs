use std::io::{stdin, Read};
use itertools::Itertools;

type LobbyResult<'a, T> = Result<T, LobbyError<'a>>;
type LobbyMultiResult<'a, T> = Result<Vec<T>, Vec<LobbyError<'a>>>;

#[derive(Debug, PartialEq)]
enum LobbyError<'a> {
    InvalidInput(&'a str),
}

const DAY: u8 = 3;

fn main() {
    let mut data = String::new();
    stdin().read_to_string(&mut data).unwrap();

    let battery_banks = match parse_battery_banks(&data) {
        Ok(v) => v,
        Err(errors) => {
            eprintln!("Cannot read battery banks:");
            eprintln!("{errors:#?}");
            return;
        }
    };

    println!("Max possible voltage: {}", find_total_largest_voltage(battery_banks.clone(), 2));
}

fn find_total_largest_voltage(battery_banks: Vec<Vec<u8>>, count: usize) -> u32 {
    battery_banks
        .into_iter()
        .map(|bank| find_largest_possible_voltage(bank, count))
        .sum()
}

fn find_largest_possible_voltage(battery_bank: Vec<u8>, count: usize) -> u32 {
    let batteries = find_largest_possible_combination(battery_bank, count);

    let voltage = batteries
        .iter()
        .fold((0, count as i32 - 1), |(accum, exponent), battery| {
            let value = *battery as u32 * 10u32.pow(exponent as u32);

            (accum + value, exponent - 1)
        });

    voltage.0
}

fn find_largest_possible_combination(battery_bank: Vec<u8>, count: usize) -> Vec<u8> {
    let mut current_start_offset = 0;
    let mut combination = Vec::new();

    for current_battery in 0..count {
        let reserved_end_offset = count - current_battery - 1;
        let max_selectable_index = battery_bank.len() - reserved_end_offset;

        let selectable_range = &battery_bank[current_start_offset..max_selectable_index];
        let max_selectable_value = selectable_range.iter().max().unwrap();

        let first_index = &battery_bank[0..max_selectable_index]
            .iter()
            .position(|x| x == max_selectable_value)
            .unwrap();

        current_start_offset = *first_index + 1;
        combination.push(*max_selectable_value);
    }

    combination
}

fn parse_battery_banks(data: &str) -> LobbyMultiResult<'_, Vec<u8>> {
    let (banks, errors): (Vec<_>, Vec<_>) = data
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| parse_line(line))
        .partition_result();

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(banks)
}

fn parse_line(line: &str) -> LobbyResult<'_, Vec<u8>> {
    let parsed = line
        .chars()
        .map(|char| char.to_digit(10).map(|digit| digit as u8))
        .flatten()
        .collect_vec();

    if parsed.len() != line.len() {
        return Err(LobbyError::InvalidInput(line));
    }

    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LobbyError::InvalidInput;

    #[test]
    fn test_parse_battery_banks_valid() {
        assert_eq!(
            parse_battery_banks("1234\n5678\n9012\n\n3456"),
            Ok(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 0, 1, 2],
                vec![3, 4, 5, 6]
            ])
        );
    }

    #[test]
    fn test_parse_battery_banks_invalid() {
        assert_eq!(
            parse_battery_banks("1234\nabcd\n5678\n90ab"),
            Err(vec![InvalidInput("abcd"), InvalidInput("90ab")])
        );
    }

    #[test]
    fn test_find_largest_combination() {
        assert_eq!(
            find_largest_possible_combination(vec![9, 8, 7, 1, 1, 7, 8, 9], 3),
            vec![9, 8, 9]
        );

        assert_eq!(
            find_largest_possible_combination(vec![9, 8, 7, 1, 1, 7, 8, 7], 3),
            vec![9, 8, 8]
        );
    }

    #[test]
    fn test_find_voltage() {
        assert_eq!(
            find_largest_possible_voltage(vec![4, 5, 6, 1, 1, 1, 8, 9], 3),
            689
        );
    }

    #[test]
    fn test_example_data() {
        let example_data = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
        let battery_banks = parse_battery_banks(example_data).unwrap();

        assert_eq!(find_total_largest_voltage(battery_banks, 2), 357);
    }
}
