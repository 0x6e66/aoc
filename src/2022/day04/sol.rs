use std::str::FromStr;

const INPUT_PATH: &str = "src/2022/day04/input.txt";
const TEST_INPUT_PATH: &str = "src/2022/day04/input_test.txt";

#[derive(Debug)]
struct ElfPair((u32, u32), (u32, u32));

impl<'a> FromStr for ElfPair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted: Vec<&str> = s.split(",").collect();
        if splitted.len() != 2 {
            println!("'{}'", s);
            return Err("Error".to_string());
        }

        let mut pair1 = (0, 0);
        let mut pair2 = (0, 0);

        for (i, pair) in splitted.iter().enumerate() {
            let pair_split: Vec<u32> = pair
                .split("-")
                .into_iter()
                .map(|x| x.parse::<u32>().expect("msg"))
                .collect();
            match i {
                0 => {
                    pair1.0 = pair_split[0];
                    pair1.1 = pair_split[1];
                }
                1 => {
                    pair2.0 = pair_split[0];
                    pair2.1 = pair_split[1];
                }
                _ => (),
            }
        }

        Ok(ElfPair(pair1, pair2))
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
    let elf_pairs = read_all::<ElfPair>(path);
    let mut sum: u32 = 0;

    for ep in elf_pairs {
        match ep {
            Err(_) => (),
            Ok(ep) => {
                if (ep.0 .0 <= ep.1 .0 && ep.0 .1 >= ep.1 .1)
                    || (ep.1 .0 <= ep.0 .0 && ep.1 .1 >= ep.0 .1)
                {
                    sum += 1;
                }
            }
        }
    }

    format!("{}", sum)
}

fn part2(path: &str) -> String {
    let elf_pairs = read_all::<ElfPair>(path);
    let mut sum: u32 = 0;

    for ep in elf_pairs {
        match ep {
            Err(_) => (),
            Ok(ep) => {
                if ((ep.0 .0 <= ep.1 .0 && ep.1 .0 <= ep.0 .1)
                    || (ep.0 .0 <= ep.1 .1 && ep.1 .1 <= ep.0 .1))
                    || (ep.0 .0 <= ep.1 .0 && ep.0 .1 >= ep.1 .1)
                    || (ep.1 .0 <= ep.0 .0 && ep.1 .1 >= ep.0 .1)
                {
                    sum += 1;
                }
            }
        }
    }

    format!("{}", sum)
}
