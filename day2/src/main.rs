use std::fs;

enum Outcome {
    Win,
    Loss,
    Draw,
}

#[derive(PartialEq, Clone, Copy)]
enum MoveOption {
    Rock,
    Paper,
    Scissors,
}

struct Move {
    own: MoveOption,
    weak: MoveOption,
    strong: MoveOption,
}

impl From<String> for Move {
    fn from(input: String) -> Self {
        match input.to_lowercase().as_str() {
            "a" | "x" => return Move { own: MoveOption::Rock, strong: MoveOption::Scissors, weak: MoveOption::Paper },
            "b" | "y" => return Move { own: MoveOption::Paper, strong: MoveOption::Rock, weak: MoveOption::Scissors },
            "c" | "z" => return Move { own: MoveOption::Scissors, strong: MoveOption::Paper, weak: MoveOption::Rock },
            _default  => panic!("Invalid move found, exiting program!"),
        }
    }
}

fn calculate_points(result: Outcome, move_used: MoveOption) -> u16 {
    let mut points = 0;

    match result {
        Outcome::Win  => points += 6,
        Outcome::Loss => points += 0,
        Outcome::Draw => points += 3
    };

    match move_used {
        MoveOption::Rock     => points += 1,
        MoveOption::Paper    => points += 2,
        MoveOption::Scissors => points += 3,
    }

    points
}

fn main() {
    let data = fs::read_to_string("input.txt")
        .expect("Unable to read file.");

    let rounds_vec = data.split("\n")
        .map(|v| v.to_string())
        .filter(|v| v != "")
        .collect::<Vec<String>>();
    
    let mut total_points = 0;
    for round in rounds_vec.iter() {
        let mut matchup = round
            .split(" ")
            .collect::<Vec<&str>>();

        let enemy_move = Move::from(matchup[0].to_string());
        let (outcome, my_move): (Outcome, MoveOption) = match matchup[1].to_lowercase().as_str() {
            "z" => (Outcome::Win, enemy_move.weak),
            "x" => (Outcome::Loss, enemy_move.strong),
            "y" => (Outcome::Draw, enemy_move.own),
            _default => panic!("Invalid outcome string!"),
        };

        total_points += calculate_points(outcome, my_move) as u64;
    }

    println!("Total points: {}", total_points);
}
