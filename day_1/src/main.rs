#![allow(unused)]

use crate::CombinationLockError::InvalidInstruction;
use itertools::Itertools;
use shared::{print_program_header, read_data};

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
    
    let password = count_zero_states(50, combination);
    
    match password {
        Ok(password) => {
            println!("The password is {password}")
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
) -> Result<usize, Vec<CombinationLockError<'_>>> {
    let mut current_state = initial_state as isize;
    let mut zeroes = 0;

    let (rotation_values, errors): (Vec<_>, Vec<_>) = combination
        .into_iter()
        .map(|instruction| parse_rotation_value(instruction))
        .partition_result();

    if !errors.is_empty() {
        return Err(errors);
    }

    for rotation_value in rotation_values {
        current_state = next_state(current_state, rotation_value);

        if current_state == 0 {
            zeroes += 1;
        }
    }

    Ok(zeroes)
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

fn next_state(current_state: isize, rotation_value: isize) -> isize {
    let wrapped_rotation_value = rotation_value % 100;
    let next_state = current_state + wrapped_rotation_value;

    let next_state_wrapped = if next_state > 99 {
        next_state - 100
    } else if next_state < 0 {
        next_state + 100
    } else {
        next_state
    };

    next_state_wrapped
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
        test_next_state_in_range:               initial=42, dist=10  => up=52, down=32,
        test_next_state_distance_exactly_100:   initial=42, dist=100 => up=42, down=42,
        test_next_state_distance_over_100:      initial=42, dist=102 => up=44, down=40,

        test_next_state_wrapped:                initial=42, dist=60  => up=2,  down=82,
        test_next_state_to_zero:                initial=50, dist=50  => up=0,  down=0,
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
            Ok(3)
        );
    }
}
