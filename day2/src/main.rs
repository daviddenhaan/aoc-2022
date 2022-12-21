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

fn calculate_points(result: Outcome, move_used: &Move) -> u16 {
    let mut points = 0;

    match result {
        Outcome::Win  => points += 6,
        Outcome::Loss => points += 0,
        Outcome::Draw => points += 3
    };

    match move_used.own {
        MoveOption::Rock     => points += 1,
        MoveOption::Paper    => points += 2,
        MoveOption::Scissors => points += 3,
    }

    points
}

impl Move {
    fn against(&self, move_: &Move) -> Outcome {
        let m = move_.own;
        if self.strong == m {
            Outcome::Win
        } else if self.weak == m {
            Outcome::Loss
        } else {
            Outcome::Draw
        }
    }
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
        let mut match_moves = Vec::<Move>::new();
        round.split(" ").for_each(
            |move_string| match_moves.push(Move::from(move_string.to_string()))
        );

        let outcome = match_moves[1].against(&match_moves[0]);
        total_points += calculate_points(outcome, &match_moves[1]) as u64;
    }

    println!("Total points: {}", total_points);
}
