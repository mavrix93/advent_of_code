use crate::day11::entities::{Item, Monkey, Monkeys};

fn execute_turn(monkey: &Monkey) -> Vec<(String, Item)> {
    let mut outbox = Vec::new();
    for item in &monkey.items {
        let new_item = item.apply_operation(&monkey.operation).divide(3);
        outbox.push(((monkey.pass_test)(&new_item), new_item));
    }
    // monkey.items.clear();
    outbox
}

fn send_items(outbox: Vec<(String, Item)>, monkeys: &mut Monkeys) {
    for (receiver, item) in outbox {
        monkeys
            .get_monkey_by_name(&receiver)
            .expect(format!("No monkey found: {}", receiver).as_str())
            .add_item(item);
    }
}

pub fn execute_turns(monkeys: &mut Monkeys) {
    monkeys.0.iter().for_each(|monkey| {
        let outbox = execute_turn(monkey);
        monkey.items.clear();
        send_items(outbox, monkeys);
    });
}

pub fn execute_n_turns(monkeys: &mut Monkeys, n: usize) {
    for _ in 0..n {
        execute_turns(monkeys);
    }
}
