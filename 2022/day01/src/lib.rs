pub fn part1(input: String) -> i32 {
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

pub struct Elf {
    _id: i32,
    calories: i32,
}

pub fn part2(input: String) -> i32 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn part1_works() {
        let input = read_to_string("./short.txt").unwrap();
        let short_result = part1(input);
        assert_eq!(short_result, 24000);

        let input = read_to_string("./puzzle.txt").unwrap();
        let puzzle_result = part1(input);
        assert_eq!(puzzle_result, 69501);
    }

    #[test]
    fn part2_works() {
        let input = read_to_string("./short.txt").unwrap();
        let short_result = part2(input);
        assert_eq!(short_result, 45000);

        let input = read_to_string("./puzzle.txt").unwrap();
        let puzzle_result = part2(input);
        assert_eq!(puzzle_result, 202346);
    }
}
