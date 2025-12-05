use std::io::{stdin, Read};
use itertools::Itertools;

const DAY: u8 = 5;

#[derive(Debug, PartialEq)]
struct CafeteriaData {
    pub fresh_ranges: Vec<(u64, u64)>,
    pub available_ingredients: Vec<u64>,
}

fn main() {
    //let data = shared::read_data(DAY);
    let mut data = String::new();
    stdin().read_to_string(&mut data).unwrap();

    let parsed = parse_input(&data);
    
    let available_fresh_ingredients = find_available_fresh_ingredients(&parsed);
    println!("Available fresh ingredients: {}", available_fresh_ingredients.len());
    
    let all_fresh_ingredients = find_all_fresh_ingredients(&parsed);
    println!("All fresh ingredients: {}", all_fresh_ingredients.len());
}

fn find_available_fresh_ingredients(data: &CafeteriaData) -> Vec<u64> {
    data.available_ingredients.iter()
        .filter(|ingredient| {
            data.fresh_ranges.iter().any(|(start, end)| start <= ingredient && *ingredient <= end)
        })
        .map(|ingredient| *ingredient)
        .collect_vec()
}

fn find_all_fresh_ingredients(data: &CafeteriaData) -> Vec<u64> {
    data.fresh_ranges.iter()
        .flat_map(|(start, end)| (*start..=*end).collect_vec())
        .unique()
        .collect_vec()
}

// Technically not the full format spec, but (assuming correct input data) it's close enough and I want to
// complete this day faster than usual.
fn parse_input(data: &str) -> CafeteriaData {
    let (range_lines, available_lines): (Vec<_>, Vec<_>) = data
        .lines()
        .filter(|l| !l.trim().is_empty())
        .partition(|l| l.contains("-"));

    let fresh_ranges = range_lines.into_iter()
        .map(|l| {
            let split = l.split("-").collect_vec();
            (split[0].parse::<u64>().unwrap(), split[1].parse::<u64>().unwrap())
        })
        .collect_vec();

    let available_ingredients = available_lines.into_iter()
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
        assert_eq!(
            find_available_fresh_ingredients(&parsed),
            vec![5, 11, 17]
        )
    }

    #[test]
    fn test_find_all_fresh_ingredients() {
        let parsed = parse_input(EXAMPLE_INPUT);
        assert_eq!(
            find_all_fresh_ingredients(&parsed).into_iter().sorted().collect_vec(),
            vec![3,4,5,10,11,12,13,14,15,16,17,18,19,20]
        )
    }
}