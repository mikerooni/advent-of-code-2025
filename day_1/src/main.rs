#![allow(unused)]

use crate::CombinationLockError::InvalidInstruction;
use itertools::Itertools;
use shared::{print_program_header, read_data, read_data_stdin};

type CombinationLockResult<'a, T> = Result<T, CombinationLockError<'a>>;

#[derive(Debug, PartialEq)]
enum CombinationLockError<'a> {
    InvalidInstruction(&'a str),
}

// ------------------------------------------------------------------------------------------------------------------ //

fn main() {
    print_program_header(1, "Secret Entrance");
    
    let data = read_data(1);
    let combination = data.lines().into_iter()
        .filter(|line| !line.is_empty())
        .collect_vec();
    
    let result = count_zero_states(50, combination);
    
    match result {
        Ok((password, step2_password)) => {
            println!("The password is {password}");
            println!("The password for step 2 is {step2_password}");
        }
        Err(errors) => {
            eprintln!("Cannot read instructions:");
            eprintln!("{errors:#?}");
        }
    }
}

fn count_zero_states(
    initial_state: usize,
    combination: Vec<&str>,
) -> Result<(usize, usize), Vec<CombinationLockError<'_>>> {
    let mut current_state = initial_state as isize;
    let mut zeroes_including_passed = 0;
    let mut zeroes = 0;

    let (rotation_values, errors): (Vec<_>, Vec<_>) = combination
        .into_iter()
        .map(|instruction| parse_rotation_value(instruction))
        .partition_result();

    if !errors.is_empty() {
        return Err(errors);
    }

    for rotation_value in rotation_values {
        let (next_state, passed_zeroes) = next_state(current_state, rotation_value);
        current_state = next_state;
        zeroes_including_passed += passed_zeroes as usize;

        if current_state == 0 {
            zeroes += 1;
            zeroes_including_passed += 1;
        }
    }

    Ok((zeroes, zeroes_including_passed))
}

fn parse_rotation_value(instruction: &str) -> CombinationLockResult<'_, isize> {
    if instruction.len() < 2 {
        return Err(InvalidInstruction(instruction));
    }

    let direction = &instruction[0..1];
    let distance: usize = (&instruction[1..])
        .parse()
        .map_err(|_| InvalidInstruction(instruction))?;

    let direction_multiplier: isize = match direction {
        "L" | "l" => Ok(-1),
        "R" | "r" => Ok(1),
        _ => Err(InvalidInstruction(instruction)),
    }?;

    Ok((distance as isize) * direction_multiplier)
}

fn next_state(current_state: isize, rotation_value: isize) -> (isize, isize) {
    let mut passed_zeroes = (rotation_value as f64 / 100.0).abs().floor() as isize;
    let wrapped_rotation_value = rotation_value % 100;
    let next_state = current_state + wrapped_rotation_value;

    let next_state_wrapped = if next_state > 99 {
        if next_state - 100 != 0 {
            passed_zeroes += 1;
        }
        next_state - 100
    } else if next_state < 0 {
        if current_state != 0 {
            passed_zeroes += 1;
        }
        next_state + 100
    } else {
        next_state
    };

    (next_state_wrapped, passed_zeroes)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! rotation_parser_tests {
        ($($test_name:ident: $value:expr => $expected:expr,)*) => {$(
            #[test]
            fn $test_name() {
                ::core::assert_eq!(super::parse_rotation_value($value), $expected);
            }
        )*};
    }

    rotation_parser_tests! {
        test_rotation_left_uppercase:       "L42" => Ok(-42),
        test_rotation_left_lowercase:       "l42" => Ok(-42),

        test_rotation_right_uppercase:      "R42" => Ok(42),
        test_rotation_right_lowercase:      "r42" => Ok(42),

        test_rotation_empty_instruction:    ""    => Err(InvalidInstruction("")),
        test_rotation_invalid_direction:    "X1"  => Err(InvalidInstruction("X1")),
        test_rotation_missing_direction:    "1"   => Err(InvalidInstruction("1")),
        test_rotation_invalid_number:       "XY"  => Err(InvalidInstruction("XY")),
        test_rotation_missing_number:       "L"   => Err(InvalidInstruction("L")),
    }

    macro_rules! fold_state_tests {
        ($($test_name:ident: initial = $initial_value:expr, dist = $distance:expr => up = $expected_add:expr, down = $expected_sub:expr,)*) => {$(
            #[test]
            fn $test_name() {
                ::core::assert_eq!(super::next_state($initial_value, $distance), $expected_add);
                ::core::assert_eq!(super::next_state($initial_value, -$distance), $expected_sub);
            }
        )*};
    }

    fold_state_tests! {
        test_next_state_in_range:               initial=42, dist=10  => up=(52, 0), down=(32, 0),
        test_next_state_distance_exactly_100:   initial=42, dist=100 => up=(42, 1), down=(42, 1),
        test_next_state_distance_over_100:      initial=42, dist=102 => up=(44, 1), down=(40, 1),
        test_next_state_distance_over_300:      initial=42, dist=302 => up=(44, 3), down=(40, 3),

        test_next_state_wrapped:                initial=42, dist=60  => up=(2, 1),  down=(82, 1),
        test_next_state_to_zero:                initial=50, dist=50  => up=(0, 0),  down=(0, 0),
    }

    #[test]
    fn test_example_sequence() {
        assert_eq!(
            count_zero_states(
                50,
                vec![
                    "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82"
                ]
            ),
            Ok((3, 6))
        );
    }
}
