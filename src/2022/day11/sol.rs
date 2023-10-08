use std::{convert::Infallible, fmt::Debug, ops::Deref, str::FromStr};

const INPUT_PATH: &str = "src/2022/day11/input.txt";
const TEST_INPUT_PATH: &str = "src/2022/day11/input_test.txt";

struct Monkey {
    items: Vec<u32>,
    number_of_inspected_items: u32,
    operation: Box<dyn Fn(u32) -> u32>,
    div_test: u32,
    test_true: u32,
    test_false: u32,
}

macro_rules! impl_div_and_true_false {
    ($name:ident, $num:expr, $it:ident) => {
        let $name: u32 = $it
            .get($num)
            .unwrap()
            .as_ref()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse()
            .unwrap();
    };
}

impl Monkey {
    fn new(lines: Vec<Result<String, Infallible>>) -> Self {
        let starting_items = lines
            .get(1)
            .unwrap()
            .as_ref()
            .unwrap()
            .clone()
            .split(":")
            .collect::<Vec<&str>>()[1]
            .split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| {
                x.trim()
                    .parse::<u32>()
                    .expect("Could not parse starting items")
            })
            .collect::<Vec<u32>>();

        let op = lines.get(2).unwrap().as_ref().unwrap().clone();
        let op = op.split(" ").collect::<Vec<&str>>();
        let op: Box<dyn Fn(u32) -> u32> = match op[7] {
            "old" => Box::new(move |old| old * old),
            s => match (op[6], s.parse::<u32>().unwrap()) {
                ("*", n) => Box::new(move |old| old * n),
                ("+", n) => Box::new(move |old| old + n),
                _ => unreachable!(),
            },
        };

        impl_div_and_true_false!(div_test, 3, lines);
        impl_div_and_true_false!(test_true, 4, lines);
        impl_div_and_true_false!(test_false, 5, lines);

        Monkey {
            items: starting_items,
            number_of_inspected_items: 0,
            operation: op,
            div_test: div_test,
            test_true: test_true,
            test_false: test_false,
        }
    }
}

fn main() {
    println!("Part 1 Solution: '{}'", part1(TEST_INPUT_PATH));
    // println!("Part 2 Solution: '{}'", part2(INPUT_PATH));
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

    let mut monkeys: Vec<Monkey> = vec![];

    for m in lines.chunks(7) {
        monkeys.push(Monkey::new(m.to_vec()));
    }

    // for (i, m) in monkeys.iter().enumerate() {
    //     println!("Monkey {}", i);
    //     println!("  Items: {:?}", m.items);
    //     println!(
    //         "  Example ops (1,2,3,4,5): {:?}",
    //         [
    //             (m.operation)(1),
    //             (m.operation)(2),
    //             (m.operation)(3),
    //             (m.operation)(4),
    //             (m.operation)(5)
    //         ]
    //     );
    //     println!("  div_test: {:?}", m.div_test);
    //     println!("  test_true: {:?}", m.test_true);
    //     println!("  test_true: {:?}", m.test_false);
    // }

    let mut list_of_item_lists: Vec<Vec<u32>> = vec![vec![]; monkeys.len()];

    (0..20).for_each(|r| {
        monkeys.iter_mut().enumerate().for_each(|(i, monkey)| {
            // println!("Monkey {i}: {} {:?} {:?}", monkey.number_of_inspected_items, monkey.items, list_of_item_lists[i]);
            list_of_item_lists[i].iter().for_each(|item| {
                monkey.items.push(*item);
            });
            list_of_item_lists[i].clear();
            monkey.items.iter_mut().for_each(|item| {
                // println!("{} {}",i,  item);
                let new_worry_level = ((monkey.operation)(*item) as f32 / 3.0).floor() as u32;
                monkey.number_of_inspected_items += 1;
                match new_worry_level % monkey.div_test == 0 {
                    true => list_of_item_lists[monkey.test_true as usize].push(new_worry_level),
                    false => list_of_item_lists[monkey.test_false as usize].push(new_worry_level),
                }
            });
            monkey.items.clear();
        });

        println!("Round {}", r+1);
        monkeys.iter().enumerate().for_each(|(i,m)| {
            println!("Monkey {i}: {} {:?}", m.number_of_inspected_items, list_of_item_lists[i]);
        });
        println!();
    });

    let mut tmp: Vec<u32> = monkeys
        .iter_mut()
        .map(|m| m.number_of_inspected_items)
        .collect();
    tmp.sort_by(|a, b| b.cmp(a));

    println!("{:?}", tmp.iter().take(2).product::<u32>());

    format!("{}", 0)
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
