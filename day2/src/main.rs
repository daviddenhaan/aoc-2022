use std::fs;

enum Outcome {
    Win,
    Loss,
    Draw,
}

enum MoveOption {
    Rock,
    Paper,
    Scissors,
}

struct Move {
    move_: MoveOption,
}

impl From<String> for Move {
    fn from(input: String) -> Self {
        match input.to_lowercase().as_str() {
            "a" | "x" => return Move { move_: MoveOption::Rock },
            "b" | "y" => return Move { move_: MoveOption::Paper },
            "c" | "z" => return Move { move_: MoveOption::Scissors },
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

    match move_used.move_ {
        MoveOption::Rock     => points += 1,
        MoveOption::Paper    => points += 2,
        MoveOption::Scissors => points += 3,
    }

    points
}

impl Move {
    fn against(&self, move_: &Move) -> Outcome {
        match move_.move_ {
            MoveOption::Rock     => {
                match self.move_ {
                    MoveOption::Paper    => Outcome::Win,
                    MoveOption::Scissors => Outcome::Loss,
                    MoveOption::Rock     => Outcome::Draw,
                }
            },
            MoveOption::Paper    => {
                match self.move_ {
                    MoveOption::Scissors => Outcome::Win,
                    MoveOption::Rock     => Outcome::Loss,
                    MoveOption::Paper    => Outcome::Draw,
                }

            },
            MoveOption::Scissors => {
                match self.move_ {
                    MoveOption::Rock     => Outcome::Win,
                    MoveOption::Paper    => Outcome::Loss,
                    MoveOption::Scissors => Outcome::Draw,
                }
            },
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
