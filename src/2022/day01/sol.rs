use std::str::FromStr;

const INPUT_PATH: &str = "src/2022/day01/input.txt";
const TEST_INPUT_PATH: &str = "src/2022/day01/input_test.txt";

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
    let lines = read_all::<u32>(path);
    // println!("{:?}", lines);

    let mut calories_per_elf: Vec<u32> = vec![];

    let mut tmp_elf: u32 = 0;

    for line in lines {
        match line {
            Err(_) => {
                calories_per_elf.push(tmp_elf);
                tmp_elf = 0;
            },
            Ok(cals) => {
                tmp_elf += cals;
            }
        }
    }
    calories_per_elf.push(tmp_elf);
    format!("{}", calories_per_elf.iter().max().unwrap())
}

fn part2(path: &str) -> String {
    let lines = read_all::<u32>(path);
    // println!("{:?}", lines);

    let mut calories_per_elf: Vec<u32> = vec![];
    let mut tmp_elf: u32 = 0;

    for line in lines {
        match line {
            Err(_) => {
                calories_per_elf.push(tmp_elf);
                tmp_elf = 0;
            },
            Ok(cals) => {
                tmp_elf += cals;
            }
        }
    }
    calories_per_elf.push(tmp_elf);

    calories_per_elf.sort();
    
    let mut sum = 0;
    for i in calories_per_elf.len() - 3..calories_per_elf.len() {
        sum += calories_per_elf[i];
    }
    format!("{}", sum)
}
