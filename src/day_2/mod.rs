use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

// https://adventofcode.com/2021/day/2

#[derive(Debug)]
struct Puzzle1Position {
    horizontal: isize,
    depth: isize,
}

#[derive(Debug)]
struct Puzzle2Position {
    horizontal: isize,
    depth: isize,
    aim: isize,
}

#[derive(Debug)]
struct Command {
    command: String,
    magnitude: isize,
}

pub fn puzzle_2() -> isize {
    let commands: Vec<Command> = parse_input("./inputs/day_2_puzzle_1.txt");

    let mut current_position = Puzzle2Position {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };

    for command in commands {
        match command.command.as_str() {
            "up" => {
                current_position = Puzzle2Position {
                    horizontal: current_position.horizontal,
                    depth: current_position.depth,
                    aim: current_position.aim - command.magnitude,
                }
            }
            "down" => {
                current_position = Puzzle2Position {
                    horizontal: current_position.horizontal,
                    depth: current_position.depth,
                    aim: current_position.aim + command.magnitude,
                }
            }
            "forward" => {
                current_position = Puzzle2Position {
                    horizontal: current_position.horizontal + command.magnitude,
                    depth: current_position.depth + (current_position.aim * command.magnitude),
                    aim: current_position.aim,
                }
            }
            _ => panic!("Unexpected command received"),
        }
    }

    current_position.horizontal * current_position.depth
}

pub fn puzzle_1() -> isize {
    let commands: Vec<Command> = parse_input("./inputs/day_2_puzzle_1.txt");

    let mut current_position = Puzzle1Position {
        horizontal: 0,
        depth: 0,
    };

    for command in commands {
        match command.command.as_str() {
            "up" => {
                current_position = Puzzle1Position {
                    horizontal: current_position.horizontal,
                    depth: current_position.depth - command.magnitude,
                }
            }
            "down" => {
                current_position = Puzzle1Position {
                    horizontal: current_position.horizontal,
                    depth: current_position.depth + command.magnitude,
                }
            }
            "forward" => {
                current_position = Puzzle1Position {
                    horizontal: current_position.horizontal + command.magnitude,
                    depth: current_position.depth,
                }
            }
            _ => panic!("Unexpected command"),
        }
    }

    current_position.horizontal * current_position.depth
}

fn parse_cmd(raw_cmd: &str) -> Command {
    let cmd_parts: Vec<&str> = raw_cmd.split_whitespace().collect();

    Command {
        command: cmd_parts[0].to_string(),
        magnitude: isize::from_str(cmd_parts[1]).unwrap(),
    }
}

fn parse_input(input_path: &str) -> Vec<Command> {
    let f = File::open(input_path).unwrap();
    let f = BufReader::new(f);

    f.lines()
        .map(|line| parse_cmd(line.unwrap().as_str()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_1() {
        assert_eq!(puzzle_1(), 1499229);
    }

    #[test]
    fn test_puzzle_2() {
        assert_eq!(puzzle_2(), 1340836560);
    }
}
