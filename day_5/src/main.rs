use itertools::{Itertools, sorted, merge};
use std::cmp::{max, min};
use std::io::{Read, stdin};

const DAY: u8 = 5;

#[derive(Debug, PartialEq)]
struct CafeteriaData {
    pub fresh_ranges: Vec<(u64, u64)>,
    pub available_ingredients: Vec<u64>,
}

fn main() {
    let data = shared::read_data(DAY);

    let parsed = parse_input(&data);

    let available_fresh_ingredients = find_available_fresh_ingredients(&parsed);
    println!(
        "Available fresh ingredients: {}",
        available_fresh_ingredients.len()
    );

    let all_fresh_ingredients = count_all_fresh_ingredients(parsed);
    println!("All fresh ingredients: {}", all_fresh_ingredients);

}

fn find_available_fresh_ingredients(data: &CafeteriaData) -> Vec<u64> {
    data.available_ingredients
        .iter()
        .filter(|ingredient| {
            data.fresh_ranges
                .iter()
                .any(|(start, end)| start <= ingredient && *ingredient <= end)
        })
        .map(|ingredient| *ingredient)
        .collect_vec()
}

fn count_all_fresh_ingredients(data: CafeteriaData) -> u64 {
    let merged_ranges = combine_overlapping_ranges(data.fresh_ranges);
    let total_count = merged_ranges.iter().fold(0, |acc, range| acc + (range.1 - range.0) + 1);

    total_count
}


fn combine_overlapping_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut iteration = 0;

    loop {
        println!("Iteration: {}", iteration);
        iteration += 1;

        let (updated_count, combined) = combine_overlapping_ranges_single_iteration(ranges.clone());

        if updated_count == 0 {
            break;
        }

        ranges = combined;
    }

    ranges
}

fn combine_overlapping_ranges_single_iteration(ranges: Vec<(u64, u64)>) -> (u64, Vec<(u64, u64)>) {

    let mut combined: Vec<(u64, u64)> = Vec::new();
    let mut updated_count = 0;

    for range in ranges {
        match combined.iter_mut().find(|existing| ranges_overlap(&range, *existing)) {
            None => {
                combined.push(range)
            }
            Some((start, end)) => {
                updated_count += 1;

                *start = min(range.0, *start);
                *end = max(range.1, *end);
            }
        }
    }

    (updated_count, combined)
}

fn ranges_overlap(a: &(u64, u64), b: &(u64, u64)) -> bool {
    if a.0 <= b.0 && b.0 <= a.1 {
        return true;
    }

    if b.0 <= a.0 && a.0 <= b.1 {
        return true;
    }

    false
}

// Technically not the full format spec, but (assuming correct input data) it's close enough and I want to
// complete this day faster than usual.
fn parse_input(data: &str) -> CafeteriaData {
    let (range_lines, available_lines): (Vec<_>, Vec<_>) = data
        .lines()
        .filter(|l| !l.trim().is_empty())
        .partition(|l| l.contains("-"));

    let fresh_ranges = range_lines
        .into_iter()
        .map(|l| {
            let split = l.split("-").collect_vec();
            (
                split[0].parse::<u64>().unwrap(),
                split[1].parse::<u64>().unwrap(),
            )
        })
        .collect_vec();

    let available_ingredients = available_lines
        .into_iter()
        .map(|l| l.parse::<u64>().unwrap())
        .collect_vec();

    println!("ranges: {fresh_ranges:?}");
    println!("available ingredients: {available_ingredients:?}");

    CafeteriaData {
        fresh_ranges,
        available_ingredients,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(EXAMPLE_INPUT),
            CafeteriaData {
                fresh_ranges: vec![(3, 5), (10, 14), (16, 20), (12, 18)],
                available_ingredients: vec![1, 5, 8, 11, 17, 32]
            }
        );
    }

    #[test]
    fn test_find_available_fresh_ingredients() {
        let parsed = parse_input(EXAMPLE_INPUT);
        assert_eq!(find_available_fresh_ingredients(&parsed), vec![5, 11, 17])
    }

    #[test]
    fn test_find_all_fresh_ingredients() {
        let parsed = parse_input(EXAMPLE_INPUT);
        assert_eq!(count_all_fresh_ingredients(parsed), 14)
    }

    #[test]
    fn test_combine_overlapping_ranges_separate() {
        assert_eq!(
            combine_overlapping_ranges(vec![(1, 2)]),
            vec![(1, 2)]
        );
    }

    #[test]
    fn test_combine_overlapping_ranges_overlap_start() {
        assert_eq!(
            combine_overlapping_ranges(vec![(1, 2), (4, 5), (6, 10)]),
            vec![(1, 2), (4, 5), (6, 10)]
        );
    }

    #[test]
    fn test_combine_overlapping_ranges_overlap_end() {
        assert_eq!(
            combine_overlapping_ranges(vec![(1, 2), (2, 5), (10, 15), (12, 20)]),
            vec![(1, 5), (10, 20)]
        );
    }

    #[test]
    fn test_combine_overlapping_ranges_overlap_full_first_larger() {
        assert_eq!(
            combine_overlapping_ranges(vec![(1, 10), (3, 5)]),
            vec![(1, 10)]
        );
    }

    #[test]
    fn test_combine_overlapping_ranges_overlap_full_second_larger() {
        assert_eq!(
            combine_overlapping_ranges(vec![(3, 5), (1, 10)]),
            vec![(1, 10)]
        );
    }

    #[test]
    fn test_combine_overlapping_ranges_multi() {
        assert_eq!(
            combine_overlapping_ranges(vec![(3, 5), (10, 14), (16, 20), (12, 18)]),
            vec![(3, 5), (10, 20)]
        );
    }

    #[test]
    fn test_ranges_overlap() {
        assert_eq!(ranges_overlap(&(1, 2), &(3, 4)), false);

        assert_eq!(ranges_overlap(&(1, 2), &(2, 4)), true);
        assert_eq!(ranges_overlap(&(2, 4), &(1, 2)), true);

        assert_eq!(ranges_overlap(&(1, 3), &(2, 4)), true);
        assert_eq!(ranges_overlap(&(2, 4), &(1, 3)), true);

        assert_eq!(ranges_overlap(&(1, 10), &(2, 5)), true);
        assert_eq!(ranges_overlap(&(2, 5), &(1, 10)), true);
    }
}
