use super::board::Board;
use std::fs;

#[derive(Debug)]
pub struct Game {
    boards: Vec<Board>,
    bingo_numbers: Vec<usize>,
}

impl Game {
    pub fn from_file(input_path: &str) -> Game {
        let input = fs::read_to_string(input_path).unwrap();

        // split the input into sections based on our knowledge of the file format
        let sections: Vec<&str> = input.split("\n\n").collect();

        // name the sections for clarity
        let raw_bingo_numbers = &sections[0];
        let raw_boards = &sections[1..];

        // parse the bingo numbers
        let bingo_numbers: Vec<usize> = raw_bingo_numbers
            .split(',')
            .map(|digit| digit.parse::<usize>().unwrap())
            .collect();

        // parse the boards
        let boards: Vec<Board> = raw_boards
            .iter()
            .map(|raw_board| Board::from_raw(raw_board))
            .collect();

        Game {
            boards,
            bingo_numbers,
        }
    }

    pub fn run_to_first_winner(&mut self) -> Result<usize, &str> {
        for num in &self.bingo_numbers {
            for board in &mut self.boards {
                board.mark(*num);
                if board.is_winner() {
                    return Ok(board.score() * num);
                }
            }
        }

        Err("No winner found")
    }

    pub fn run_to_last_winner(&mut self) -> usize {
        let mut winners = vec![];

        for num in &self.bingo_numbers {
            for (i, board) in &mut self.boards.iter_mut().enumerate() {
                let before = board.is_winner();

                board.mark(*num);

                let after = board.is_winner();

                if before != after {
                    winners.push((num, i));
                }
            }

            // stop processing bingo numbers if all the boards have won
            if self.boards.iter().all(super::board::Board::is_winner) {
                break;
            }
        }

        let (winning_num, board_idx) = winners.last().unwrap();
        let last_winning_board_score = self.boards.get(*board_idx).unwrap().score();

        last_winning_board_score * *winning_num
    }
}
