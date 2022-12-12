use std::fmt::Debug;

#[derive(Debug)]
pub struct Item(pub i32);

impl Item {
    pub fn apply_operation(&self, operation: &impl Fn(i32) -> i32) -> Self {
        Self(operation(self.0))
    }
}

pub struct Monkey {
    pub name: String,
    pub items: Vec<Item>,
    pub operation: Box<dyn Fn(i32) -> i32>,
    pub pass_test: Box<dyn Fn(&Item) -> String>,
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("name", &self.name)
            .field("items", &self.items)
            .field("operation", &"Box<dyn Fn(i32) -> i32>")
            .field("pass_test", &"Box<dyn Fn(&Item) -> String>")
            .finish()
    }
}

impl Monkey {
    pub fn new(
        name: String,
        operation: impl Fn(i32) -> i32 + 'static,
        pass_test: impl Fn(&Item) -> String + 'static,
        items: Vec<Item>,
    ) -> Self {
        Self {
            name,
            items,
            operation: Box::new(operation),
            pass_test: Box::new(pass_test),
        }
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    // pub fn execute_operations(&self) -> String {
    //     let mut new
    //     for item in &self.items {
    //         result = (self.operation)(result + item.0);
    //     }
    //     (self.pass_test)(result)
    // }
}

#[derive(Debug)]
pub struct Monkeys<'a>(pub Vec<&'a mut Monkey>);

impl<'a> Monkeys<'a> {
    pub fn get_monkey_by_name(&self, name: &str) -> Option<&&'a mut Monkey> {
        self.0.iter().find(|monkey| monkey.name == name)
    }
}
