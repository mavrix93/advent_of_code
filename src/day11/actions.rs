use crate::day11::entities::{Item, Monkey, Monkeys};

fn execute_turn(monkey: &mut Monkey, monkeys: &mut Monkeys) {
    for item in &monkey.items {
        let new_item = item.apply_operation(&monkey.operation).divide(3);
        send_item((monkey.pass_test)(&new_item), new_item, monkeys);
    }
    monkey.items.clear();
}

fn send_item(receiver: String, item: Item, monkeys: &mut Monkeys) {
    monkeys
        .get_monkey_by_name(&receiver)
        .expect(format!("No monkey found: {}", receiver).as_str())
        .add_item(item);
}

pub fn execute_turns(monkeys: &mut Monkeys) {
    monkeys
        .0
        .iter_mut()
        .for_each(|monkey| execute_turn(monkey, monkeys));
}

pub fn execute_n_turns(monkeys: &mut Monkeys, n: usize) {
    for _ in 0..n {
        execute_turns(monkeys);
    }
}
