use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct Monkey {
    pub items: RefCell<Vec<u64>>,
    pub operation: char,
    pub operation_input: u64,
    pub test_number: u64,
    pub test_true_monkey: u64,
    pub test_false_monkey: u64
}

impl Monkey {
    pub fn inspect_items(&self, worry_fn: impl Fn(u64) -> u64) -> Vec<u64> {
        let new_items = self.items.borrow().iter().map(|item| {
            return worry_fn(Self::change_worry_level(&self, *item));
        }).collect();

        self.items.borrow_mut().clear();

        return new_items;
    }

    fn change_worry_level(&self, item: u64) -> u64 {
        let input = match self.operation_input {
            0 => item,
            _ => self.operation_input
        };

        return match self.operation {
            '*' => match item.checked_mul(input) {
                Some(v) => v,
                None => {
                    println!("cannot multiply {} {}", item, input);
                    panic!()
                }
            }, 
            '+' => (item + input) as u64,
            _ => panic!()
        }
    }

    pub fn test_item(&self, item: u64) -> u64 {
        if item % self.test_number == 0 {
            return self.test_true_monkey;
        } else {
            return self.test_false_monkey;
        }
    }

    pub fn throw_at_monkey(&self, item: u64, monkey: &Monkey) {
        monkey.receive_from_monkey(item);
    }

    fn receive_from_monkey(&self, item: u64) {
        self.items.borrow_mut().push(item);
    }
}

