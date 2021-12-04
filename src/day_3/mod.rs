use std::fs;

pub fn puzzle_1() -> usize {
    let raw_input = parse_input("./inputs/day_3_puzzle_1.txt");
    let rows = build_rows(&raw_input);
    let cols = build_cols_from_rows(rows, 12);

    let gamma_rate = calc_gamma_rate(cols);
    let epsilon_rate = calc_epsilon_rate(gamma_rate, 12);

    gamma_rate * epsilon_rate
}

fn parse_input(input_path: &str) -> String {
    fs::read_to_string(input_path).expect("Failed to read file")
}

fn build_rows(raw_input: &str) -> Vec<Vec<usize>> {
    raw_input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|diagnostic_line| {
            diagnostic_line
                .chars()
                .map(|chr| chr.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

// Build a vector of columns from the rows (we are interested in computing over
// the values in a single column)
fn build_cols_from_rows(rows: Vec<Vec<usize>>, num_cols: usize) -> Vec<Vec<usize>> {
    let mut cols: Vec<Vec<usize>> = vec![vec![]; num_cols];

    for row in rows {
        for i in 0..row.len() {
            cols[i].push(row[i]);
        }
    }

    cols
}

fn calc_gamma_rate(cols: Vec<Vec<usize>>) -> usize {
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

fn calc_epsilon_rate(gamma_rate: usize, significant_bits: u32) -> usize {
    // Dynamically create a bit mask with all 1's for the lower
    // `significant_bits` bits e.g.
    //
    //   significant_bits | mask
    //   ---------------- | ----
    //   12               | 0xFFF
    //   5                | 0x1F

    let mask = 2_usize.pow(significant_bits) - 1;

    // Only the lowest `significant_bits` bits are significant in `gamma_rate`.
    // Bit flip it and then AND with a mask to reset all bits higher than
    // `significant_bits` to 0
    !gamma_rate & mask
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
    fn test_build_cols() {
        let rows = build_rows(EXAMPLE_INPUT);
        let result = build_cols_from_rows(rows, 5);

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
        let cols = build_cols_from_rows(rows, 5);
        let result = calc_gamma_rate(cols);

        let expected = 22;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_calc_epsilon_rate() {
        let gamma_rate = 22;
        let result = calc_epsilon_rate(gamma_rate, 5);

        let expected = 9;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_puzzle_1() {
        assert_eq!(puzzle_1(), 1307354);
    }
}
