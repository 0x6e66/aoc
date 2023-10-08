use std::str::FromStr;

const INPUT_PATH: &str = "src/2022/day03/input.txt";
const TEST_INPUT_PATH: &str = "src/2022/day03/input_test.txt";

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
    let mut sum: u32 = 0;

    for line in lines {
        match line {
            Err(_) => (),
            Ok(line) => {
                let (first, second) = line.split_at(line.len() / 2);
                'outer: for c1 in first.chars() {
                    for c2 in second.chars() {
                        if c1 == c2 {
                            sum += match c1.is_lowercase() {
                                true => (c1 as u32) - 96,
                                false => (c1 as u32) - 38,
                            };
                            break 'outer;
                        }
                    }
                }
            }
        }
    }

    format!("{}", sum)
}

fn part2(path: &str) -> String {
    let lines = read_all::<String>(path);
    let mut sum: u32 = 0;

    for group in lines.chunks(3) {
        'outer: for c1 in group[0].clone().unwrap().chars() {
            for c2 in group[1].clone().unwrap().chars() {
                for c3 in group[2].clone().unwrap().chars() {
                    if c1 == c2 && c2 == c3 {
                        sum += match c1.is_lowercase() {
                            true => (c1 as u32) - 96,
                            false => (c1 as u32) - 38,
                        };
                        break 'outer;
                    }
                }
            }
        }
    }

    format!("{}", sum)
}
