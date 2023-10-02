use std::{collections::HashMap, str::FromStr};

const INPUT_PATH: &str = "src/2022/day02/input.txt";
const TEST_INPUT_PATH: &str = "src/2022/day02/input_test.txt";

#[derive(Debug)]
struct RPS {
    l: char,
    r: char,
}

impl<'a> FromStr for RPS {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted: Vec<&str> = s.split(" ").collect();
        if splitted.len() != 2 {
            println!("'{}'", s);
            return Err("Error".to_string());
        }

        Ok(RPS {
            l: splitted[0].chars().next().unwrap(),
            r: splitted[1].chars().next().unwrap(),
        })
    }
}

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
    let lines = read_all::<RPS>(path);

    let strategy_map: HashMap<(char, char), (u32, u32)> = HashMap::from([
        (('A', 'X'), (1, 3)),
        (('A', 'Y'), (2, 6)),
        (('A', 'Z'), (3, 0)),
        (('B', 'X'), (1, 0)),
        (('B', 'Y'), (2, 3)),
        (('B', 'Z'), (3, 6)),
        (('C', 'X'), (1, 6)),
        (('C', 'Y'), (2, 0)),
        (('C', 'Z'), (3, 3)),
    ]);

    let mut total_score: u32 = 0;

    for line in lines {
        let rps = line.unwrap();
        let (shape_score, outcome_score) = strategy_map[&(rps.l, rps.r)];
        total_score += shape_score + outcome_score;
    }
    format!("{}", total_score)
}

fn part2(path: &str) -> String {
    let lines = read_all::<RPS>(path);

    let strategy_map: HashMap<(char, char), (u32, u32)> = HashMap::from([
        (('A', 'X'), (3, 0)),
        (('A', 'Y'), (1, 3)),
        (('A', 'Z'), (2, 6)),
        (('B', 'X'), (1, 0)),
        (('B', 'Y'), (2, 3)),
        (('B', 'Z'), (3, 6)),
        (('C', 'X'), (2, 0)),
        (('C', 'Y'), (3, 3)),
        (('C', 'Z'), (1, 6)),
    ]);

    let mut total_score: u32 = 0;

    for line in lines {
        let rps = line.unwrap();
        let (shape_score, outcome_score) = strategy_map[&(rps.l, rps.r)];
        total_score += shape_score + outcome_score;
    }
    format!("{}", total_score)
}
