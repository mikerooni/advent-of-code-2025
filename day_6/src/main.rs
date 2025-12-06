use itertools::Itertools;
use std::io::{stdin, Read};

const DAY: u8 = 6;

fn main() {
    //let data = shared::read_data(DAY);
    let mut data = String::new();
    stdin().read_to_string(&mut data).unwrap();

    let parsed = match parse_input(&data) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Error while parsing data: {e:?}");
            return;
        }
    };

    let results = parsed.iter().map(|p| p.solve()).collect_vec();
    println!("Total: {}", results.iter().sum::<u64>());


    let parsed_2 = parse_input_part_2(&data);
    let results_2 = parsed_2.iter().map(|p| p.solve()).collect_vec();
    println!("Total (part 2): {}", results_2.iter().sum::<u64>())
}

#[derive(Debug, PartialEq)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug, PartialEq)]
struct MathProblem {
    numbers: Vec<u64>,
    operation: Operation,
}

#[derive(Debug, PartialEq)]
enum MathProblemParserError<'a> {
    MismatchedColumns,
    UnknownOperation(&'a str),
}

impl MathProblem {
    fn solve(&self) -> u64 {
        self.numbers
            .iter()
            .skip(1)
            .fold(self.numbers[0], |acc, x| match self.operation {
                Operation::Add => acc + x,
                Operation::Multiply => acc * x,
            })
    }
}

fn parse_input(data: &str) -> Result<Vec<MathProblem>, MathProblemParserError<'_>> {
    let input_table: Vec<Vec<&str>> = data
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.split(" ")
                .filter(|part| !part.is_empty())
                .collect_vec()
        })
        .collect_vec();

    let column_count = input_table.iter().map(Vec::len).unique().collect_vec();
    if column_count.len() != 1 {
        return Err(MathProblemParserError::MismatchedColumns);
    }

    let row_count = input_table.len();
    let column_count = column_count[0];

    let operation_row = input_table.last().unwrap();
    if let Some(invalid_op) = operation_row.iter().find(|op| **op != "+" && **op != "*") {
        return Err(MathProblemParserError::UnknownOperation(invalid_op));
    }

    let problems = (0..column_count)
        .map(|column| {
            let numbers = input_table
                .iter()
                .take(row_count - 1)
                .map(|row| row[column].parse::<u64>().expect("Numbers should be valid"))
                .collect_vec();

            let operation = match operation_row[column] {
                "+" => Operation::Add,
                "*" => Operation::Multiply,
                _ => panic!("Previously checked operations should not be invalid"),
            };

            MathProblem { numbers, operation }
        })
        .collect_vec();

    Ok(problems)
}

fn parse_input_part_2(data: &str) -> Vec<MathProblem> {
    let number_row_count = data.lines().count() - 1;
    let number_table = data
        .lines()
        .take(number_row_count)
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let operation_row = data.lines().last().unwrap().chars().collect_vec();
    let column_ranges = extract_column_ranges(&operation_row);

    let mut problems = Vec::new();
    for (col_min, col_max) in column_ranges {
        let numbers = parse_col_numbers(col_min, col_max, &number_table);
        let operation = match operation_row[col_min] {
            '+' => Operation::Add,
            '*' => Operation::Multiply,
            _ => panic!("Invalid operation"),
        };
        problems.push(MathProblem { numbers, operation });
    }

    problems
}

fn parse_col_numbers(col_min: usize, col_max: usize, number_table: &Vec<Vec<char>>) -> Vec<u64> {
    let numbers = (col_min..=col_max)
        .rev()
        .map(|col| {
            let (_, number) = number_table
                .iter()
                .fold((0, String::new()), |(idx, acc), row| match row[col] {
                    ' ' => (idx, acc),
                    c => (idx + 1, format!("{}{}", acc, c)),
                });
            number
        })
        .map(|num| num.parse::<u64>().expect("Numbers should be valid"))
        .collect_vec();

    numbers
}

fn extract_column_ranges(operation_row: &Vec<char>) -> Vec<(usize, usize)> {
    let mut ranges: Vec<(usize, usize)> = Vec::new();

    let mut current_idx = 0;

    // index:  0123456789
    // data:   *    +   +
    for c in operation_row {
        if (*c == '+' || *c == '*') && current_idx != 0 {
            let range_start = ranges.last().map(|r| r.1 + 2).unwrap_or(0);
            ranges.push((range_start, current_idx - 2));
        }

        current_idx += 1;
    }

    let range_start = ranges.last().map(|r| r.1 + 2).unwrap_or(0);
    ranges.push((range_start, current_idx - 1));

    ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str =
        "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

    #[test]
    fn test_parse_example_input() {
        let parsed = parse_input(EXAMPLE_INPUT);

        assert_eq!(
            parsed,
            Ok(vec![
                MathProblem {
                    numbers: vec![123, 45, 6],
                    operation: Operation::Multiply
                },
                MathProblem {
                    numbers: vec![328, 64, 98],
                    operation: Operation::Add
                },
                MathProblem {
                    numbers: vec![51, 387, 215],
                    operation: Operation::Multiply
                },
                MathProblem {
                    numbers: vec![64, 23, 314],
                    operation: Operation::Add
                },
            ])
        );

        let results = parsed.unwrap().iter().map(|p| p.solve()).collect_vec();
        assert_eq!(results, vec![33210, 490, 4243455, 401]);
    }

    #[test]
    fn test_parse_example_input_part_2() {
        let parsed = parse_input_part_2(EXAMPLE_INPUT);

        assert_eq!(
            parsed,
            vec![
                MathProblem {
                    numbers: vec![356, 24, 1],
                    operation: Operation::Multiply
                },
                MathProblem {
                    numbers: vec![8, 248, 369],
                    operation: Operation::Add
                },
                MathProblem {
                    numbers: vec![175, 581, 32],
                    operation: Operation::Multiply
                },
                MathProblem {
                    numbers: vec![4, 431, 623],
                    operation: Operation::Add
                },
            ]
        );

        let results = parsed.iter().map(|p| p.solve()).collect_vec();
        assert_eq!(results, vec![8544, 625, 3253600, 1058]);
        assert_eq!(results.iter().sum::<u64>(), 3263827);
    }

    #[test]
    fn test_extract_column_ranges() {
        assert_eq!(
            extract_column_ranges(&"*    +   +      *   ".chars().collect_vec()),
            vec![(0, 3), (5, 7), (9, 14), (16, 19)]
        );
    }

    #[test]
    fn test_column_ranges_example_data() {
        /*
        idx:             11111
               012345678901234
               ---------------
        data:  123 328  51 64
                45 64  387 23
                 6 98  215 314
        */

        assert_eq!(
            extract_column_ranges(&EXAMPLE_INPUT.lines().last().unwrap().chars().collect_vec()),
            vec![(0, 2), (4, 6), (8, 10), (12, 14)]
        );
    }

    #[test]
    fn test_parse_col_numbers() {
        let number_table = EXAMPLE_INPUT
            .lines()
            .take(3)
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        /*
        idx:             11111
               012345678901234
               ---------------
        data:  123 328  51 64
                45 64  387 23
                 6 98  215 314
        */
        assert_eq!(parse_col_numbers(0, 2, &number_table), vec![356, 24, 1]);
    }
}
