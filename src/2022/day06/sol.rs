use std::{collections::HashSet, str::FromStr};

const INPUT_PATH: &str = "src/2022/day06/input.txt";
const TEST_INPUT_PATH: &str = "src/2022/day06/input_test.txt";

fn main() {
    println!("Part 1 Solution: '{}'", part1(INPUT_PATH));
    println!("Part 2 Solution: '{}'", part2(INPUT_PATH));
}

fn read_all<T: FromStr>(file_name: &str) -> Vec<Result<T, <T as FromStr>::Err>> {
    std::fs::read_to_string(file_name)
        .expect(format!("File '{file_name}' not found!").as_str())
        .lines()
        .map(|x| x.parse())
        .collect()
}

fn part1(path: &str) -> String {
    let lines = read_all::<String>(path);

    let mut marker_positions: Vec<usize> = vec![];

    for line in lines {
        match line {
            Err(_) => (),
            Ok(line) => {
                for i in 0..line.len() - 3 {
                    let mut set: HashSet<char> = HashSet::new();
                    set.insert(line.chars().nth(i).unwrap());
                    set.insert(line.chars().nth(i + 1).unwrap());
                    set.insert(line.chars().nth(i + 2).unwrap());
                    set.insert(line.chars().nth(i + 3).unwrap());
                    if set.len() == 4 {
                        marker_positions.push(i + 4);
                        break;
                    }
                }
            }
        }
    }

    format!("{}", marker_positions.first().unwrap())
}

fn part2(path: &str) -> String {
    let lines = read_all::<String>(path);

    let mut marker_positions: Vec<usize> = vec![];

    for line in lines {
        match line {
            Err(_) => (),
            Ok(line) => {
                for i in 0..line.len() - 13 {
                    let mut set: HashSet<char> = HashSet::new();
                    for j in 0..14 {
                        set.insert(line.chars().nth(i + j).unwrap());
                    }
                    if set.len() == 14 {
                        marker_positions.push(i + 14);
                        break;
                    }
                }
            }
        }
    }

    format!("{}", marker_positions.first().unwrap())
}
