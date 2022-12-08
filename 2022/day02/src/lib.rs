pub fn part1(input: String) -> i32 {
    let final_score = input.lines().fold(0, |acc, l| -> i32 {
        if l.chars().count() < 3 {
            println!("didn't get 3 values here ...");
            return acc
        }

        match l.chars().collect::<Vec<char>>().as_slice() {
            ['A', _, 'X'] => acc + 1 + 3, // rock vs rock
            ['A', _, 'Y'] => acc + 2 + 6, // rock vs paper
            ['A', _, 'Z'] => acc + 3 + 0, // rock vs scissors
            ['B', _, 'X'] => acc + 1 + 0, // paper vs rock
            ['B', _, 'Y'] => acc + 2 + 3, // paper vs paper
            ['B', _, 'Z'] => acc + 3 + 6, // paper vs scissors
            ['C', _, 'X'] => acc + 1 + 6, // scissors vs rock
            ['C', _, 'Y'] => acc + 2 + 0, // scissors vs paper
            ['C', _, 'Z'] => acc + 3 + 3, // scissors vs scissors
            _ => unreachable!(),
        }
    });
    println!("{}", final_score);
    return final_score;
}

pub fn part2(input: String) -> i32 {
    let final_score = input.lines().fold(0, |acc, l| -> i32 {
        if l.chars().count() < 3 {
            println!("didn't get 3 values here ...");
            return acc
        }

        match l.chars().collect::<Vec<char>>().as_slice() {
            ['A', _, 'X'] => acc + 3 + 0, // rock / lose -> scissors
            ['A', _, 'Y'] => acc + 1 + 3, // rock / draw -> rock
            ['A', _, 'Z'] => acc + 2 + 6, // rock / win -> paper
            ['B', _, 'X'] => acc + 1 + 0, // paper / lose -> rock
            ['B', _, 'Y'] => acc + 2 + 3, // paper / draw -> paper
            ['B', _, 'Z'] => acc + 3 + 6, // paper / win -> scissors
            ['C', _, 'X'] => acc + 2 + 0, // scissors / lose -> paper
            ['C', _, 'Y'] => acc + 3 + 3, // scissors / draw -> scissors
            ['C', _, 'Z'] => acc + 1 + 6, // scissors / win -> rock
            _ => unreachable!(),
        }
    });
    println!("{}", final_score);
    return final_score;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn part1_works() {
        let input = read_to_string("./short.txt").unwrap();
        let result = part1(input);
        assert_eq!(result, 15);

        let input = read_to_string("./puzzle.txt").unwrap();
        let result = part1(input);
        assert_eq!(result, 13446);
    }

    #[test]
    fn part2_works() {
        let input = read_to_string("./short.txt").unwrap();
        let result = part2(input);
        assert_eq!(result, 12);

        let input = read_to_string("./puzzle.txt").unwrap();
        let result = part2(input);
        assert_eq!(result, 13509);
    }
}
