use std::{str::FromStr, convert::Infallible};

const INPUT_PATH: &str = "src/2022/day05/input.txt";
const TEST_INPUT_PATH: &str = "src/2022/day05/input_test.txt";

#[derive(Debug)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

impl<'a> FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted: Vec<&str> = s.split(" ").collect();

        if splitted.len() != 6 {
            return Err("err".to_owned());
        }

        let amount = match splitted[1].parse::<usize>() {
            Err(e) => return Err(e.to_string()),
            Ok(amount) => amount
        };
        let from = match splitted[3].parse::<usize>() {
            Err(e) => return Err(e.to_string()),
            Ok(from) => from
        };
        let to = match splitted[5].parse::<usize>() {
            Err(e) => return Err(e.to_string()),
            Ok(to) => to
        };

        Ok(Move {
            amount: amount,
            from: from-1,
            to: to-1,
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

fn get_stacks(lines: &Vec<Result<String, Infallible>>) -> Vec<Vec<String>> {
    let mut stacks: Vec<Vec<String>> = vec![];

    for line in lines {
        match line {
            Err(_) => (),
            Ok(line) => {
                if line.is_empty() {
                    break;
                }
                let splitted: Vec<&str> = line.split("").collect();
                for (i, b) in splitted.chunks(4).enumerate() {
                    match b.len() {
                        4 => {
                            if i >= stacks.len() {
                                stacks.push(vec![]);
                            }
                            if !b[2].trim().is_empty() {
                                stacks[i].push(b[2].to_owned());
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
    }

    for stack in &mut stacks {
        stack.pop();
        stack.reverse();
    }

    stacks
}

fn part1(path: &str) -> String {
    let lines = read_all::<String>(path);

    let mut stacks: Vec<Vec<String>> = get_stacks(&lines);

    for line in lines {
        match line {
            Err(_) => (),
            Ok(line) => {
                let move_ = Move::from_str(line.as_str());
                match move_ {
                    Err(_) => (),
                    Ok(move_) => {
                        (0..move_.amount).into_iter().for_each(|_| {
                            let val = stacks[move_.from].pop().unwrap();
                            stacks[move_.to].push(val);
                        });
                    }
                }
            }
        }
    }

    let mut res = String::new();

    for s in &mut stacks {
        res.push(s.pop().unwrap().chars().collect::<Vec<char>>()[0]);
    }

    format!("{}", res)
}

fn part2(path: &str) -> String {
    let lines = read_all::<String>(path);

    let mut stacks: Vec<Vec<String>> = get_stacks(&lines);

    for line in lines {
        match line {
            Err(_) => (),
            Ok(line) => {
                let move_ = Move::from_str(line.as_str());
                match move_ {
                    Err(_) => (),
                    Ok(move_) => {
                        let mut tmp_stack: Vec<String> = vec![];
                        (0..move_.amount).into_iter().for_each(|_| {
                            let val = stacks[move_.from].pop().unwrap();
                            tmp_stack.push(val);
                        });
                        tmp_stack.reverse();
                        for val in tmp_stack {
                            stacks[move_.to].push(val);
                        }
                    }
                }
            }
        }
    }

    let mut res = String::new();

    for s in &mut stacks {
        res.push(s.pop().unwrap().chars().collect::<Vec<char>>()[0]);
    }

    format!("{}", res)
}
