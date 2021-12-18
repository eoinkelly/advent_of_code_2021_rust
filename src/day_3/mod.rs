use std::fs;

pub fn puzzle_1() -> usize {
    let raw_input = parse_input("./inputs/day_3_puzzle_1.txt");
    let rows = build_rows(&raw_input);
    let row_len = rows.get(0).expect("Failed to read first row").len();
    let cols = build_cols_from_rows(&rows);
    let gamma_rate = puzzle_1_gamma_rate(&cols);
    let epsilon_rate = puzzle_1_epsilon_rate(gamma_rate, row_len);

    gamma_rate * epsilon_rate
}

pub fn puzzle_2() -> usize {
    let raw_input = parse_input("./inputs/day_3_puzzle_1.txt");
    let rows = build_rows(&raw_input);
    let row_len = rows.get(0).expect("Failed to read first row").len();
    let o2_rate = o2_generator_rating(&rows, row_len);
    let co2_scrubber_rate = co2_scrubber_rating(&rows, row_len);

    o2_rate * co2_scrubber_rate
}

fn parse_input(input_path: &str) -> String {
    fs::read_to_string(input_path).expect("Failed to read file")
}

fn build_rows(raw_input: &str) -> Vec<Vec<usize>> {
    raw_input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|chr| chr.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

// Build a vector of columns from the rows
//
// We are interested in computing over the values in a single column.
//
fn build_cols_from_rows(rows: &[Vec<usize>]) -> Vec<Vec<usize>> {
    // We assume that all rows are the same length
    let num_cols = rows.get(0).expect("Failed to read first row").len();

    let mut cols: Vec<Vec<usize>> = vec![vec![]; num_cols];

    for row in rows {
        for i in 0..num_cols {
            cols[i].push(row[i]);
        }
    }

    cols
}

fn puzzle_1_gamma_rate(cols: &[Vec<usize>]) -> usize {
    let mut gamma_rate = 0;

    for col in cols {
        let count_ones: usize = col.iter().sum();
        let count_zeros = col.len() - count_ones;

        // sanity check
        assert_eq!(count_ones + count_zeros, col.len());

        if count_ones >= count_zeros {
            // if there are more 1's than 0's then we "shift in a 1" by
            // left-shift the output and setting least significant bit to 1
            gamma_rate <<= 1;
            gamma_rate |= 1;
        } else {
            // if there are more 1's than 0's then we "shift in a 0" by just
            // left-shifting the output
            gamma_rate <<= 1;
        }
    }

    gamma_rate
}

#[allow(clippy::cast_possible_truncation)]
fn puzzle_1_epsilon_rate(gamma_rate: usize, significant_bits: usize) -> usize {
    // Dynamically create a bit mask with all 1's for the lower
    // `significant_bits` bits e.g.
    //
    //   significant_bits | mask
    //   ---------------- | ----
    //   12               | 0xFFF
    //   5                | 0x1F
    let mask = 2_usize.pow(significant_bits as u32) - 1;

    // Only the lowest `significant_bits` bits are significant in `gamma_rate`.
    // Bit flip it and then AND with a mask to reset all bits higher than
    // `significant_bits` to 0
    !gamma_rate & mask
}

// o2 rating is whatever there is more of in each position
// fn o2_generator_rating(rows: &Vec<Vec<usize>>, num_bits: usize) -> usize {
fn o2_generator_rating(rows: &[Vec<usize>], num_bits: usize) -> usize {
    let mut answers = rows.to_owned();

    for i in 0..num_bits {
        let cols = build_cols_from_rows(&answers);
        let (ones, zeros): (Vec<usize>, Vec<usize>) = cols[i].iter().partition(|&&n| n == 1);

        if ones.len() >= zeros.len() {
            // keep only rows with 1 in this position
            answers = answers.into_iter().filter(|row| row[i] == 1).collect();
        } else {
            // keep only rows with 0 in this position
            answers = answers.into_iter().filter(|row| row[i] == 0).collect();
        }

        if answers.len() <= 1 {
            break;
        }
    }

    convert_num(&answers[0])
}

fn co2_scrubber_rating(rows: &[Vec<usize>], num_bits: usize) -> usize {
    let mut answers = rows.to_owned();

    for i in 0..num_bits {
        let cols = build_cols_from_rows(&answers);
        let (ones, zeros): (Vec<usize>, Vec<usize>) = cols[i].iter().partition(|&&n| n == 1);

        if ones.len() >= zeros.len() {
            // keep only rows with 1 in this position
            answers = answers.into_iter().filter(|row| row[i] == 0).collect();
        } else {
            // keep only rows with 0 in this position
            answers = answers.into_iter().filter(|row| row[i] == 1).collect();
        }

        if answers.len() <= 1 {
            break;
        }
    }

    convert_num(&answers[0])
}

#[allow(clippy::cast_possible_truncation)]
fn convert_num(digits: &[usize]) -> usize {
    let mut result: usize = 0;

    for (i, digit) in digits.iter().rev().enumerate() {
        if *digit == 1 {
            result += 2_usize.pow(i as u32);
        }
    }

    result as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";

    #[test]
    fn test_convert_nums() {
        let input = vec![1, 1, 1, 1, 0];
        let expected = 0b11110; // 30
        let result = convert_num(&input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_build_rows() {
        let result = build_rows(EXAMPLE_INPUT);
        let expected = vec![
            vec![0, 0, 1, 0, 0],
            vec![1, 1, 1, 1, 0],
            vec![1, 0, 1, 1, 0],
            vec![1, 0, 1, 1, 1],
            vec![1, 0, 1, 0, 1],
            vec![0, 1, 1, 1, 1],
            vec![0, 0, 1, 1, 1],
            vec![1, 1, 1, 0, 0],
            vec![1, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 1],
            vec![0, 0, 0, 1, 0],
            vec![0, 1, 0, 1, 0],
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_o2_generator_rating() {
        let rows = build_rows(EXAMPLE_INPUT);
        let result = o2_generator_rating(&rows, 5);
        let expected = 23;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_co2_scrubber_rating() {
        let rows = build_rows(EXAMPLE_INPUT);
        let result = co2_scrubber_rating(&rows, 5);
        let expected = 10;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_build_cols() {
        let rows = build_rows(EXAMPLE_INPUT);
        let result = build_cols_from_rows(&rows);

        let expected = vec![
            vec![0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
            vec![0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1],
            vec![0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0],
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_calc_gamma_rate() {
        let rows = build_rows(EXAMPLE_INPUT);
        let cols = build_cols_from_rows(&rows);
        let result = puzzle_1_gamma_rate(&cols);

        let expected = 22;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_calc_epsilon_rate() {
        let gamma_rate = 22;
        let result = puzzle_1_epsilon_rate(gamma_rate, 5);

        let expected = 9;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_puzzle_1() {
        assert_eq!(puzzle_1(), 1307354);
    }
}
