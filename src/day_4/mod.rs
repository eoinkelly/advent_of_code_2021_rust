mod board;
mod game;
mod square;

use game::Game;

pub fn puzzle_1() -> usize {
    first_winning_board_score_from("./inputs/day_4_puzzle.txt")
}

pub fn puzzle_2() -> usize {
    last_winning_board_score_from("./inputs/day_4_puzzle.txt")
}

pub fn first_winning_board_score_from(input_file_path: &str) -> usize {
    let mut game = Game::from_file(input_file_path);

    game.run_to_first_winner().unwrap()
}

pub fn last_winning_board_score_from(input_file_path: &str) -> usize {
    let mut game = Game::from_file(input_file_path);

    game.run_to_last_winner()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_4_puzzle_1_example() {
        let expected = 4512;
        let result = first_winning_board_score_from("./inputs/day_4_example.txt");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_day_4_puzzle_1_real() {
        let expected = 12796;
        let result = first_winning_board_score_from("./inputs/day_4_puzzle.txt");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_day_4_puzzle_2_example() {
        let expected = 1924;
        let result = last_winning_board_score_from("./inputs/day_4_example.txt");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_day_4_puzzle_2_real() {
        let expected = 18063;
        let result = last_winning_board_score_from("./inputs/day_4_puzzle.txt");
        assert_eq!(result, expected);
    }
}
