use crate::day11::entities::{Item, Monkey, Monkeys};
use std::fs::File;
use std::io::{self, Read};

pub fn load_monkeys(mut file: File) -> Monkeys {
    let mut monkeys = Vec::new();

    let mut file_text = String::new();
    file.read_to_string(&mut file_text).unwrap();

    let mut line_parts = file_text.trim().split("\n");

    loop {
        match parse_monkey(&mut line_parts) {
            Some(monkey) => monkeys.push(monkey),
            None => break,
        }
    }

    Monkeys(monkeys)
}

fn parse_monkey(parts: &mut std::str::Split<&str>) -> Option<Monkey> {
    match parts.next().map(|s| s.trim()) {
        None => None,
        Some(monkey_name_line) => Some({
            let monkey_name = monkey_name_line.split_once(":").unwrap().0;
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

            let operation = _get_operation(
                parts
                    .next()
                    .unwrap()
                    .trim()
                    .split_once(":")
                    .unwrap()
                    .1
                    .trim(),
            );

            let _test_operation_number = parts
                .next()
                .unwrap()
                .trim()
                .split_once("Test: divisible by ")
                .unwrap()
                .1
                .parse::<i32>()
                .unwrap();
            let _true_test_operation_receiver = parts
                .next()
                .unwrap()
                .trim()
                .split_once("monkey")
                .unwrap()
                .1
                .trim()
                .parse::<i32>()
                .unwrap();
            let _false_test_operation_receiver = parts
                .next()
                .unwrap()
                .trim()
                .split_once("monkey")
                .unwrap()
                .1
                .trim()
                .parse::<i32>()
                .unwrap();

            Monkey::new(
                monkey_name.to_string(),
                operation,
                move |x: &Item| {
                    if (x.0 % _test_operation_number) == 0 {
                        _true_test_operation_receiver.to_string()
                    } else {
                        _false_test_operation_receiver.to_string()
                    }
                },
                item_values,
            )
        }),
    }
}

fn _get_operation(operation: &str) -> Box<dyn Fn(i32) -> i32> {
    match operation.split_once("+") {
        Some((_, b)) => {
            let b = b.trim().parse::<i32>().unwrap();
            Box::new(|x: i32| x + b)
        }
        None => match operation.split_once("*") {
            Some((_, b)) => {
                let b = b.trim().parse::<i32>().unwrap();
                Box::new(|x: i32| x * b)
            }
            None => panic!("Invalid operation"),
        },
    }
}
