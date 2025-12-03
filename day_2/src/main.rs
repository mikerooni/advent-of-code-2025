use itertools::Itertools;
use shared::{print_program_header, read_data};
use std::cmp::{max, min};
use std::collections::HashMap;
use std::num::ParseIntError;

const DAY: u8 = 2;

type GiftShopResult<'a, T> = Result<T, GiftShopError<'a>>;
type GiftShopMultiResult<'a, T> = Result<Vec<T>, Vec<GiftShopError<'a>>>;

#[derive(Debug, PartialEq)]
enum GiftShopError<'a> {
    InvalidRange(&'a str),
}

fn main() {
    print_program_header(DAY, "Secret Entrance");

    let data = read_data(DAY);
    let ranges = match parse_all_ranges(&data) {
        Ok(v) => v,
        Err(errors) => {
            eprintln!("Cannot read ranges:");
            eprintln!("{errors:#?}");
            return;
        }
    };

    println!(
        "The sum of all invalid IDs for 2 repetitions is {}",
        sum_all_invalid_ids(ranges.clone(), vec![2])
    );

    let max_repetitions = find_max_possible_repetitions(&ranges);
    let all_repetitions_sum: u128 =
        sum_all_invalid_ids(ranges.clone(), (2..=max_repetitions).collect_vec());

    println!("The sum of all invalid IDs for all possible repetitions is {all_repetitions_sum}",);
}

fn find_max_possible_repetitions(ranges: &Vec<(u64, u64)>) -> u64 {
    let max_value = ranges.iter().map(|&(a, b)| b).max().unwrap_or(0);
    let max_len = max_value.ilog10() as u64 + 1;

    max(max_len, 2)
}

fn sum_all_invalid_ids(ranges: Vec<(u64, u64)>, repeat_counts: Vec<u64>) -> u128 {
    let invalid_ids = ranges
        .into_iter()
        .flat_map(|range| {
            repeat_counts
                .iter()
                .flat_map(|repeats| find_invalid_ids_in_range(range, *repeats))
                .collect_vec()
        })
        .unique();

    invalid_ids.sum()
}

fn find_invalid_ids_in_range(range: (u64, u64), repeats: u64) -> Vec<u128> {
    let potential_parts = find_potential_partials(range, repeats);
    let invalid_ids = repeat_partials(potential_parts, repeats);

    let filtered_invalid_ids = invalid_ids
        .clone()
        .into_iter()
        .filter(|partial| *partial >= range.0 as u128 && *partial <= range.1 as u128)
        .collect_vec();

    filtered_invalid_ids
}

fn find_potential_partials(range: (u64, u64), repeats: u64) -> Vec<u64> {
    let min_length = (range.0.ilog10() + 1) as u64;
    let max_length = (range.1.ilog10() + 1) as u64;

    let potential_lengths = (min_length..=max_length).filter(|length| length % repeats == 0);

    let potential_partials = potential_lengths.flat_map(|len| {
        let partial_len = len / repeats;
        let split_divisor = 10u64.pow((len - partial_len) as u32);
        let min_value = max(range.0 / split_divisor, 10u64.pow((partial_len - 1) as u32));
        let max_value = min(range.1 / split_divisor, 10u64.pow((partial_len) as u32) - 1);

        min_value..=max_value
    });

    potential_partials.collect_vec()
}

fn repeat_partials(partials: Vec<u64>, repeats: u64) -> Vec<u128> {
    let mut multipliers_by_len = HashMap::new();

    partials
        .iter()
        .map(|partial| repeat_partial(*partial, repeats, &mut multipliers_by_len))
        .collect_vec()
}

fn repeat_partial(partial: u64, repeats: u64, multipliers_by_len: &mut HashMap<u64, u64>) -> u128 {
    let len = partial.ilog10() as u64;
    let base_multiplier = 10u64.pow(len as u32 + 1);

    let multiplier = *multipliers_by_len.entry(len).or_insert_with(|| {
        (0..repeats).fold(0, |accum, repeat| {
            accum + base_multiplier.pow(repeat as u32)
        })
    });

    partial as u128 * multiplier as u128
}

fn parse_all_ranges(data: &str) -> GiftShopMultiResult<'_, (u64, u64)> {
    let (ranges, errors): (Vec<_>, Vec<_>) = data
        .lines()
        .filter(|line| !line.is_empty())
        .flat_map(|line| line.split(","))
        .map(|range| parse_range(range))
        .partition_result();

    if errors.is_empty() {
        Ok(ranges)
    } else {
        Err(errors)
    }
}

fn parse_range(range: &str) -> GiftShopResult<(u64, u64)> {
    if range.is_empty() {
        return Err(GiftShopError::InvalidRange(range));
    }

    let parts = range.split("-").collect_vec();
    if parts.len() != 2 {
        return Err(GiftShopError::InvalidRange(range));
    }

    let Ok((start, end)) = parse_range_parts(parts[0], parts[1]) else {
        return Err(GiftShopError::InvalidRange(range));
    };

    Ok((start, end))
}

fn parse_range_parts(start: &str, end: &str) -> Result<(u64, u64), ParseIntError> {
    Ok((start.parse()?, end.parse()?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GiftShopError::InvalidRange;

    #[test]
    fn test_parse_range_valid() {
        assert_eq!(parse_range("1-2"), Ok((1, 2)))
    }

    #[test]
    fn test_parse_range_invalid() {
        assert_eq!(parse_range(""), Err(InvalidRange("")));
        assert_eq!(parse_range("1-"), Err(InvalidRange("1-")));
        assert_eq!(parse_range("1 2"), Err(InvalidRange("1 2")));
        assert_eq!(parse_range("a-b"), Err(InvalidRange("a-b")));
    }

    #[test]
    fn test_parse_all_ranges_valid() {
        assert_eq!(
            parse_all_ranges("1-2,3-4,5-6\n7-8"),
            Ok(vec![(1, 2), (3, 4), (5, 6), (7, 8)]),
        );
    }

    #[test]
    fn test_parse_all_ranges_invalid() {
        assert_eq!(
            parse_all_ranges("1-2,3-4,5-6,a-b,d,12,7-8,-"),
            Err(vec![
                InvalidRange("a-b"),
                InvalidRange("d"),
                InvalidRange("12"),
                InvalidRange("-"),
            ])
        );
    }

    #[test]
    fn test_find_potential_parts() {
        assert_eq!(
            find_potential_partials((133332, 369295901), 2),
            (133..=9999).collect_vec()
        );

        assert_eq!(
            find_potential_partials((133332, 369295901), 3),
            (13..=369).collect_vec()
        );

        assert_eq!(
            find_potential_partials((133332, 369295901), 4),
            (10..=99).collect_vec()
        );
    }

    #[test]
    fn test_find_potential_parts_with_repeats_above_length() {
        assert_eq!(find_potential_partials((11, 22), 4), vec![]);
    }

    #[test]
    fn test_repeat_partials() {
        assert_eq!(
            repeat_partials(vec![22, 333, 4444, 55555], 3),
            vec![222222, 333333333, 444444444444, 555555555555555]
        )
    }

    #[test]
    fn test_example_ranges() {
        let example_data = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565635-565659,824824821-824824827,2121212118-2121212124";

        let example_ranges = parse_all_ranges(example_data).unwrap();
        assert_eq!(sum_all_invalid_ids(example_ranges, vec![2]), 1227775554);
    }
}
