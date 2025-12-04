use std::io::{stdin, Read};
use crate::PrintingDepartmentError::{EmptyInput, IllegalInput, MismatchedRowSize};
use itertools::Itertools;

const DAY: u8 = 4;

type PrintingDepartmentMultiResult<'a, T> = Result<Vec<T>, Vec<PrintingDepartmentError<'a>>>;
type PrintingDepartmentResult<'a, T> = Result<T, PrintingDepartmentError<'a>>;

#[derive(Debug, PartialEq)]
enum PrintingDepartmentError<'a> {
    IllegalInput(&'a str),
    MismatchedRowSize,
    EmptyInput,
}

type PaperRollRow = Vec<bool>;
type PaperRollRack = Vec<PaperRollRow>;

fn main() {
    let mut data = String::new();
    stdin().read_to_string(&mut data).unwrap();

    let rack: PaperRollRack = match parse_paper_rolls(&data) {
        Ok(rack) => rack,
        Err(errors) => {
            eprintln!("Errors occurred while parsing the input: {errors:#?}");
            return;
        }
    };

    println!("Accessible rolls: {}", count_accessible_rolls(rack, 3))
}

fn count_accessible_rolls(rack: PaperRollRack, max_occupied_adjacent: usize) -> usize {
    let mut accessible_rolls = 0;

    let width = rack[0].len();
    let height = rack.len();

    let padded_rack = pad_rack(rack);

    for row in 1..=height {
        for column in 1..=width {
            if padded_rack[row][column] == false {
                continue;
            }

            let occupied_neighbor_count = find_occupied_neighbor_count(&padded_rack, row, column);
            if occupied_neighbor_count <= max_occupied_adjacent {
                accessible_rolls += 1;
            }
        }
    }

    accessible_rolls
}

fn find_occupied_neighbor_count(padded_rack: &PaperRollRack, row: usize, column: usize) -> usize {
    let mut occupied_neighbors = 0;

    for check_row in (row - 1)..=(row + 1) {
        for check_column in (column - 1)..=(column + 1) {
            let cell_occupied = padded_rack[check_row][check_column];

            if cell_occupied && (check_row != row || check_column != column) {
                occupied_neighbors += 1;
            }
        }
    }

    occupied_neighbors
}

fn pad_rack(mut rack: PaperRollRack) -> PaperRollRack {
    rack.iter_mut().for_each(|row| {
        row.insert(0, false);
        row.push(false);
    });

    let row_len = rack[0].len();

    let mut padding_top_bottom_row = Vec::new();
    padding_top_bottom_row.resize(row_len, false);

    rack.insert(0, padding_top_bottom_row.clone());
    rack.push(padding_top_bottom_row.clone());

    rack
}

fn parse_paper_rolls(data: &str) -> PrintingDepartmentMultiResult<'_, PaperRollRow> {
    let (rows, errors): (Vec<_>, Vec<_>) = data
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| parse_input_row(line))
        .partition_result();

    if !errors.is_empty() {
        return Err(errors);
    }

    let row_length_count = rows.iter().map(|row| row.len()).unique().count();
    if row_length_count != 1 {
        return Err(vec![MismatchedRowSize]);
    }

    if rows.is_empty() {
        return Err(vec![EmptyInput]);
    }

    Ok(rows)
}

fn parse_input_row(line: &str) -> PrintingDepartmentResult<'_, PaperRollRow> {
    let (rolls, errors): (Vec<_>, Vec<_>) = line
        .chars()
        .map(|c| match c {
            '@' => Ok(true),
            '.' => Ok(false),
            _ => Err(IllegalInput(line)),
        })
        .partition_result();

    if !errors.is_empty() {
        return Err(IllegalInput(line));
    }

    Ok(rolls)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rolls_valid() {
        assert_eq!(
            parse_paper_rolls("@.@.\n@@..\n@..@\n\n.@@."),
            Ok(vec![
                vec![true, false, true, false],
                vec![true, true, false, false],
                vec![true, false, false, true],
                vec![false, true, true, false],
            ])
        )
    }

    #[test]
    fn test_parse_rolls_illegal_chars() {
        assert_eq!(
            parse_paper_rolls("@.@a\n@@..\n@  @\n\n.@@."),
            Err(vec![IllegalInput("@.@a"), IllegalInput("@  @"),])
        )
    }

    #[test]
    fn test_parse_rolls_mismatched_row_size() {
        assert_eq!(
            parse_paper_rolls("@@@@\n@@@\n@@@@"),
            Err(vec![MismatchedRowSize])
        )
    }

    #[test]
    fn test_pad_rack() {
        assert_eq!(
            pad_rack(vec![vec![true, true], vec![true, true]]),
            vec![
                vec![false, false, false, false],
                vec![false, true, true, false],
                vec![false, true, true, false],
                vec![false, false, false, false],
            ]
        );
    }

    #[test]
    fn test_example_data() {
        let example_data = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";
        let rack = parse_paper_rolls(&example_data).unwrap();

        assert_eq!(
            count_accessible_rolls(rack, 3),
            13
        );
    }
}
