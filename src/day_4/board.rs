use super::square::Square;
use crate::matrix;

#[derive(Debug)]
pub struct Board {
    squares: Vec<Vec<Square>>,
}

impl Board {
    pub fn from_raw(raw_board: &str) -> Self {
        let mut rows = vec![];

        for row in raw_board.lines() {
            let cols: Vec<Square> = row
                .split_ascii_whitespace()
                .map(|i_chr| i_chr.parse::<usize>().unwrap())
                .map(Square::new)
                .collect();
            rows.push(cols);
        }

        Self { squares: rows }
    }

    pub fn mark(&mut self, number: usize) {
        for row in &mut self.squares {
            for square in row {
                if square.value_is(number) {
                    square.mark();
                }
            }
        }
    }

    pub fn is_winner(&self) -> bool {
        // check each row
        for row in &self.squares {
            if row.iter().all(Square::is_marked) {
                return true;
            }
        }

        // check each column
        for col in matrix::transpose(&self.squares) {
            if col.iter().all(|s| s.is_marked()) {
                return true;
            }
        }

        false
    }

    // sum all unmarked numbers on board
    pub fn score(&self) -> usize {
        let mut sum = 0;

        for row in &self.squares {
            for square in row {
                if !square.is_marked() {
                    sum += square.get_value();
                }
            }
        }

        sum
    }
}
