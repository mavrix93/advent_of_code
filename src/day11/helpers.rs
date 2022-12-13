use crate::day11::entities::{Item, Monkey, Monkeys};
use std::fs::File;
use std::io::{self, Read};

pub fn load_monkeys(mut file: File) -> Monkeys {
    let mut monkeys = Vec::new();

    let mut file_text = String::new();
    file.read_to_string(&mut file_text).unwrap();

    let mut line_parts = file_text.trim().split("\n\n");

    for monkey_section in line_parts {
        monkeys.push(parse_monkey(monkey_section).unwrap());
    }
    Monkeys(monkeys)
}

fn parse_monkey(monkey_section: &str) -> Option<Monkey> {
    let mut parts = monkey_section.split("\n");

    let monkey_name = parts.next()?.trim().split_once(":")?.0;
    let item_values = parts
        .next()
        .unwrap()
        .trim()
        .rsplit_once("Starting items:")
        .unwrap()
        .1
        .split(",")
        .map(|x| Item(x.trim().parse::<i32>().unwrap()))
        .collect();

    let operation = _get_operation(parts.next()?.trim().split_once(":").unwrap().1.trim());

    let _test_operation_number = parts
        .next()?
        .trim()
        .split_once("Test: divisible by ")
        .unwrap()
        .1
        .parse::<i32>()
        .unwrap();
    let _true_test_operation_receiver = parts
        .next()?
        .trim()
        .split_once("to")
        .expect(format!("No 'to' found in {} for positive", monkey_section).as_str())
        .1
        .trim()
        .to_string();
    let _false_test_operation_receiver = parts
        .next()?
        .trim()
        .split_once("to")
        .expect(format!("No 'to' found in {} for false", monkey_section).as_str())
        .1
        .trim()
        .to_string();

    Some(Monkey::new(
        monkey_name.to_string().to_lowercase(),
        operation,
        move |x: &Item| {
            if (x.0 % _test_operation_number) == 0 {
                _true_test_operation_receiver.clone()
            } else {
                _false_test_operation_receiver.clone()
            }
        },
        item_values,
    ))
}

fn _get_operation(operation: &str) -> Box<dyn Fn(i32) -> i32> {
    match operation.split_once("+") {
        Some((_, b)) => {
            let b = b.trim().parse::<i32>().unwrap();
            Box::new(move |x: i32| x + b)
        }
        None => match operation.split_once("*") {
            Some((_, b)) => {
                if (operation.trim().matches("old").count() == 2) {
                    Box::new(move |x: i32| x * x)
                } else {
                    let b = b
                        .trim()
                        .parse::<i32>()
                        .expect(format!("b: {}", operation).as_str());
                    Box::new(move |x: i32| x * b)
                }
            }
            None => panic!("Invalid operation"),
        },
    }
}
