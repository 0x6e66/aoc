use std::str::FromStr;

const INPUT_PATH: &str = "src/2015/day02/input.txt";
const TEST_INPUT_PATH: &str = "src/2015/day02/input_test.txt";

fn main() {
    println!("Part 1 Solution: '{}'", part1(INPUT_PATH));
    println!("Part 2 Solution: '{}'", part2(INPUT_PATH));
}

fn read_all<T: FromStr>(file_name: &str) -> Vec<Result<T, <T as FromStr>::Err>> {
    std::fs::read_to_string(file_name)
        .expect("file not found!")
        .lines()
        .map(|x| x.parse())
        .collect()
}

fn part1(path: &str) -> String {
    let mut sum = 0;

    let lines = read_all::<String>(path);
    for l in lines {
        match l {
            Err(_) => (),
            Ok(content) => {
                let dims: Vec<i32> = content
                    .split("x")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect();
                let sides = vec![
                    2 * dims[0] * dims[1],
                    2 * dims[1] * dims[2],
                    2 * dims[2] * dims[0],
                ];

                let surface_area: i32 = sides.iter().sum();
                let smallest_surface: i32 = *sides.iter().min().unwrap() / 2;

                sum += surface_area + smallest_surface;
            }
        }
    }
    format!("{}", sum)
}

fn part2(path: &str) -> String {
    let mut sum = 0;

    let lines = read_all::<String>(path);
    for l in lines {
        match l {
            Err(_) => (),
            Ok(content) => {
                let dims: Vec<i32> = content
                    .split("x")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect();

                let ribbon: i32 = dims.iter().sum::<i32>() - dims.iter().max().unwrap();
                let bow: i32 = dims.iter().product();

                sum += 2 * ribbon + bow;
            }
        }
    }
    format!("{}", sum)
}
