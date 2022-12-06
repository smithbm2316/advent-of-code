use std::fs;

pub fn e1(path: &str) -> i32 {
    let input = fs::read_to_string(path).expect("Should have been able to read the file");

    let mut most_calories = 0;
    let mut calories = 0;
    let mut elf = 1;

    for line in input.split("\n") {
        if line.chars().count() == 0 {
            // println!("elf #{}, calories={}, most_calories={}", elf, calories, most_calories);
            if most_calories < calories {
                most_calories = calories;
                elf += 1;
            }
            calories = 0;
            // println!("elf #{}, calories={}, most_calories={}", elf, calories, most_calories);
        } else {
            let snack_cals: i32 = line.parse().unwrap();
            calories += snack_cals;
        }
    }

    // println!();
    println!("elf #{} was carrying this many calories: {}", elf, most_calories);
    return most_calories;
}

struct Elf {
    _id: i32,
    calories: i32,
}

pub fn e2(path: &str) -> i32 {
    let input = fs::read_to_string(path).expect("Should have been able to read the file");

    let mut current_elf: i32 = 1;
    let mut current_cals: i32 = 0;
    let mut elves: Vec<Elf> = Vec::new();

    for line in input.split("\n") {
        if line.chars().count() == 0 {
            elves.push(Elf {
                _id: current_elf,
                calories: current_cals,
            });
            current_elf += 1;
            current_cals = 0;
        } else {
            let snack_cals: i32 = line.parse().unwrap();
            current_cals += snack_cals;
        }
    }

    let mut top_elves: [Elf; 3] = [
        Elf {
            _id: -1,
            calories: 0,
        },
        Elf {
            _id: -2,
            calories: 0,
        },
        Elf {
            _id: -3,
            calories: 0,
        },
    ];

    for elf in elves {
        if elf.calories > top_elves[0].calories
            && elf.calories > top_elves[1].calories
            && elf.calories > top_elves[2].calories
        {
            top_elves.sort_by(|a, b| b.calories.cmp(&a.calories));
            top_elves[2] = elf;
        }
    }

    let total_calories = top_elves[0].calories + top_elves[1].calories + top_elves[2].calories;
    println!("total calories by top 3 elves: {}", total_calories);
    return total_calories;
}
