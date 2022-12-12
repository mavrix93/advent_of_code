use crate::day11::entities::{Item, Monkey, Monkeys};

fn execute_turn(monkey: &Monkey) -> Vec<(String, Item)> {
    let mut items_outbox = Vec::new();
    for item in &monkey.items {
        let new_item = item.apply_operation(&monkey.operation);
        items_outbox.push(((monkey.pass_test)(&new_item), new_item));
    }

    items_outbox
}

fn send_items(items_outbox: Vec<(String, Item)>, monkeys: &mut Monkeys) {
    for (monkey_name, mut item) in items_outbox {
        let mut monkey = monkeys
            .get_monkey_by_name(&monkey_name)
            .expect("No monkey found");

        monkey.add_item(item);

        monkeys
            .get_monkey_by_name(&monkey_name)
            .expect("No monkey found")
            .add_item(item);
    }
}

fn execute_turns(monkeys: &mut Monkeys) {
    for mut monkey in monkeys.0 {
        let items_outbox = execute_turn(&mut monkey);
        monkey.items.clear();
        send_items(items_outbox, &mut monkeys);
    }
}
