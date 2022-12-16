use std::fs;

#[derive(Clone)]
struct Elve {
    carrying_calories: u32,
}

impl From<&Vec<u32>> for Elve
{
    fn from(calories_data: &Vec<u32>) -> Self {
        let mut total_calories: u32 = 0;
        for v in calories_data {
            total_calories += v;
        }
        
        return Self {
            carrying_calories: total_calories,
        }
    }
}

impl Copy for Elve {}

fn main() {
    let data = fs::read_to_string("input.txt")
        .expect("Unable to read file.");

    let raw_vec = data.split("\n").collect::<Vec<&str>>();
    let mut elves: Vec<Elve> = Vec::new();

    let mut accumulate: Vec<u32> = Vec::new();
    for line in raw_vec.iter() {
        if let Ok(n) = line.parse::<u32>() {
            accumulate.push(n);
            continue;
        }

        elves.push(Elve::from(&accumulate));
        accumulate.clear();
    }

    let mut most_calories_elve: Option<Elve> = None;
    for elve in elves {
        if let None = most_calories_elve {
            most_calories_elve = Option::from(elve);
            continue;
        }

        if let Some(_max) = most_calories_elve {
            if elve.carrying_calories > _max.carrying_calories {
                most_calories_elve = Option::from(elve);
            }

            continue;
        }
    }

    if let Some(elve) = most_calories_elve {
        println!("most calories: {}", elve.carrying_calories);
    }
}
