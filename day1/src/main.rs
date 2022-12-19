use std::fs;

#[derive(Clone, Copy)]
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

    elves.sort_by(|a, b| b.carrying_calories.cmp(&a.carrying_calories));
    
    let total: u32 = elves[0..3].iter().map(|v| v.carrying_calories).sum();

    println!("top three total: {}", total);
}
