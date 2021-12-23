use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

// https://adventofcode.com/2021/day/1

pub fn puzzle_1() -> isize {
    let sonar_readings = parse_input("./inputs/day_1_puzzle_1.txt");
    let mut num_increases = 0;

    // We are counting increases in the readings. The first reading cannot count
    // as an "increase" so we skip it in our iteration
    let mut prev_reading = sonar_readings[0];

    for reading in &sonar_readings[1..] {
        if *reading > prev_reading {
            num_increases += 1;
        }
        prev_reading = *reading;
    }

    num_increases
}

pub fn puzzle_2() -> isize {
    let sonar_readings = parse_input("./inputs/day_1_puzzle_1.txt");
    let mut num_increases = 0;
    let mut prev_sum = 0;

    // For each reading, create a window of readings from itself and the next
    // two readings. We are counting increases in the sum of the window. Stop
    // iterating when there would not be enough readings left to make a 3-wide
    // window.
    for chunk in sonar_readings.windows(3) {
        let chunk_sum = chunk.iter().sum();

        if chunk_sum > prev_sum && prev_sum != 0 {
            num_increases += 1;
        } else {
        }

        prev_sum = chunk_sum;
    }

    num_increases
}

fn parse_input(input_path: &str) -> Vec<isize> {
    let f = File::open(input_path).unwrap();
    let f = BufReader::new(f);

    f.lines()
        .map(|reading| isize::from_str(reading.unwrap().as_str()).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_1() {
        assert_eq!(puzzle_1(), 1466);
    }

    #[test]
    fn test_puzzle_2() {
        assert_eq!(puzzle_2(), 1491);
    }
}
