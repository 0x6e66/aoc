const INPUT_PATH: &str = "src/2015/day01/input.txt";
const TEST_INPUT_PATH: &str = "src/2015/day01/input_test.txt";

fn main() {
    println!("Part 1 Solution: '{}'", part1(INPUT_PATH));
    println!("Part 2 Solution: '{}'", part2(INPUT_PATH));
}

fn part1(path: &str) -> String {
    let contents = std::fs::read_to_string(path).expect(format!("Couldn't read '{}'", path).as_str());
    let ups = contents.split("").into_iter().filter(|c| *c == "(").count() as i32;
    let downs = contents.split("").into_iter().filter(|c| *c == ")").count() as i32;
    format!("{}", ups - downs)
}

fn part2(path: &str) -> String {
    let contents = std::fs::read_to_string(path).expect(format!("Couldn't read '{}'", path).as_str());

    let mut floor = 0;
    let tmp = contents.split("").filter(|c| *c == "(" || *c == ")");
    for (i, c) in tmp.clone().enumerate() {
        if c == "(" {
            floor += 1;
        } else if c == ")" {
            floor -= 1;
        }

        if floor < 0 {
            return format!("{}", i + 1);
        }
    }
    format!("{}", tmp.count() - 1)
}
