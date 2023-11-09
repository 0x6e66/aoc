use std::str::FromStr;

const INPUT_PATH: &str = "src/2020/day01/input.txt";
const TEST_INPUT_PATH: &str = "src/2020/day01/input_test.txt";

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

    let mut res = 0;

    'outer: for (i, e1) in lines.iter().enumerate() {
        for (_, e2) in lines[i..].iter().enumerate() {
            let e1 = *e1.as_ref().unwrap();
            let e2 = *e2.as_ref().unwrap();
            if e1 + e2 == 2020 {
                res = e1 * e2;
                break 'outer;
            }
        }
    }

    format!("{}", res)
}

fn part2(path: &str) -> String {
    let lines = read_all::<u32>(path);

    let mut res = 0;

    'outer: for (i, e1) in lines.iter().enumerate() {
        for (j, e2) in lines[i..].iter().enumerate() {
            for (_, e3) in lines[j..].iter().enumerate() {
                let e1 = *e1.as_ref().unwrap();
                let e2 = *e2.as_ref().unwrap();
                let e3 = *e3.as_ref().unwrap();
                if e1 + e2 + e3 == 2020 {
                    res = e1 * e2 * e3;
                    break 'outer;
                }
            }
        }
    }

    format!("{}", res)
}
