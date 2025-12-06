use std::io::stdin;
use itertools::Itertools;

const DAY: u8 = 6;

fn main() {
    // let data = shared::read_data(DAY);
    let mut data = String::new();
    stdin().read_line(&mut data).unwrap();

    let parsed = match parse_input(&data) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Error while parsing data: {e:?}");
            return;
        }
    };

    let results = parsed.iter().map(|p| p.solve()).collect_vec();

    println!("Total: {}", results.iter().sum::<u64>())
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

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "123 328  51 64 \n 45 64  387 23\n  6 98  215 314\n*   +   *   +";

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
}
