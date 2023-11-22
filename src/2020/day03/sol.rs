use std::str::FromStr;

const INPUT_PATH: &str = "src/2020/day03/input.txt";
const TEST_INPUT_PATH: &str = "src/2020/day03/input_test.txt";

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

    let mut res = 0;

    lines
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, e)| (i, e.clone().unwrap()))
        .for_each(|(i, e)| {
            if e.chars().nth(i * 3 % e.len()) == Some('#') {
                res += 1;
            }
        });

    format!("{}", res)
}

fn part2(path: &str) -> String {
    let lines = read_all::<String>(path);

    let mut res = 1;
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    for (right, down) in slopes {
        let mut tmp = 0;
        lines
            .iter()
            .enumerate()
            .skip(1)
            .map(|(i, e)| (i, e.clone().unwrap()))
            .for_each(|(i, e)| {
                if i % down == 0 && e.chars().nth((i / down) * right % e.len()) == Some('#') {
                    tmp += 1;
                }
            });
        res *= tmp;
    }

    format!("{}", res)
}
