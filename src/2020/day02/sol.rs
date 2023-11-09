use std::str::FromStr;

const INPUT_PATH: &str = "src/2020/day02/input.txt";
const TEST_INPUT_PATH: &str = "src/2020/day02/input_test.txt";

struct PasswordPolicy {
    min: u32,
    max: u32,
    char: char,
    password: String,
}

impl FromStr for PasswordPolicy {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted: Vec<&str> = s.split(" ").collect();
        if splitted.len() != 3 {
            return Err(());
        }

        let minmax: Vec<&str> = splitted[0].split("-").collect();
        let min = match minmax[0].parse::<u32>() {
            Ok(min) => min,
            Err(_) => return Err(()),
        };
        let max = match minmax[1].parse::<u32>() {
            Ok(max) => max,
            Err(_) => return Err(()),
        };

        let char = match splitted[1].chars().nth(0) {
            Some(char) => char,
            None => return Err(()),
        };

        let password = splitted[2].to_string();

        Ok(PasswordPolicy {
            min,
            max,
            char,
            password,
        })
    }
}

fn main() {
    // println!("Part 1 Solution: '{}'", part1(INPUT_PATH));
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
    let pps = read_all::<PasswordPolicy>(path);

    let mut res = 0;

    pps.iter().filter_map(|pp| pp.as_ref().ok()).for_each(|pp| {
        let count = pp.password.chars().filter(|c| *c == pp.char).count() as u32;
        if pp.min <= count && count <= pp.max {
            res += 1;
        }
    });

    format!("{}", res)
}

fn part2(path: &str) -> String {
    let pps = read_all::<PasswordPolicy>(path);

    let mut res = 0;

    pps.iter().filter_map(|pp| pp.as_ref().ok()).for_each(|pp| {
        let first_char = pp.password.chars().nth((pp.min - 1) as usize).unwrap();
        let second_char = pp.password.chars().nth((pp.max - 1) as usize).unwrap();
        if (first_char == pp.char) ^ (second_char == pp.char) {
            res += 1;
        }
    });

    format!("{}", res)
}
