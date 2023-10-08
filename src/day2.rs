use crate::util;

#[derive(PartialEq, Clone, Copy)]
enum Move {
    Paper,
    Rock,
    Scissors,
}

fn get_score(my_move: Move, opponent_move: Move) -> i32 {
    let mut score = 0;
    match my_move {
        Move::Rock => score += 1,
        Move::Paper => score += 2,
        Move::Scissors => score += 3,
    }
    if my_move == opponent_move {
        score += 3;
    } else if my_move == Move::Rock && opponent_move == Move::Scissors {
        score += 6;
    } else if my_move == Move::Paper && opponent_move == Move::Rock {
        score += 6;
    } else if my_move == Move::Scissors && opponent_move == Move::Paper {
        score += 6;
    }
    score
}

pub fn part1() {
    fn get_move_from_char(symbol: char) -> Move {
        let m = match symbol {
            'A' | 'X' => Move::Rock,
            'B' | 'Y' => Move::Paper,
            'C' | 'Z' => Move::Scissors,
            _ => panic!("Invalid input"),
        };
        m
    }

    let data = util::read_file_to_string("day_2.txt".to_string());
    let mut score = 0;
    for line in data {
        let first_char = line.chars().nth(0).unwrap();
        let second_char: char = line.chars().nth(2).unwrap();

        let opponent_move = get_move_from_char(first_char);
        let my_move = get_move_from_char(second_char);

        score += get_score(my_move, opponent_move);
    }
    println!("Final score: {score}");
}

pub fn part2() {
    fn get_move_from_char(symbol: char) -> Move {
        let m = match symbol {
            'A' => Move::Rock,
            'B' => Move::Paper,
            'C' => Move::Scissors,
            _ => panic!("Invalid input"),
        };
        m
    }

    let data = util::read_file_to_string("day_2.txt".to_string());
    let mut score = 0;
    for line in data {
        let first_char = line.chars().nth(0).unwrap();
        let second_char: char = line.chars().nth(2).unwrap();

        let opponent_move = get_move_from_char(first_char);
        let my_move = match second_char {
            'X' => {
                if opponent_move == Move::Rock {
                    Move::Scissors
                } else if opponent_move == Move::Paper {
                    Move::Rock
                } else {
                    Move::Paper
                }
            }
            'Y' => opponent_move,
            'Z' => {
                if opponent_move == Move::Rock {
                    Move::Paper
                } else if opponent_move == Move::Paper {
                    Move::Scissors
                } else {
                    Move::Rock
                }
            }
            _ => panic!("Invalid input"),
        };
        score += get_score(my_move, opponent_move);
    }
    println!("Final score: {score}");
}
