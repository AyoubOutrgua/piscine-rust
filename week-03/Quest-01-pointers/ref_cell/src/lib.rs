use std::cell::RefCell;
use std::cell::Cell;
use std::rc::Rc;

pub struct Tracker {
    pub messages: RefCell<Vec<String>>,
    pub value: Cell<usize>,
    pub max: usize

}

impl Tracker {
    pub fn new(max: usize) -> Tracker {
        Tracker {
            messages: RefCell::new(Vec::<String>::new()),
            value: Cell::new(0),
            max,
        }
    }

    pub fn peek(&self, value: &Rc<usize>) {
        let count = Rc::strong_count(value);
        let calculation = (count * 100) / self.max;
        let msg: String = format!("Info: This value would use {}% of your quota", calculation);
        self.messages.borrow_mut().push(msg);
    }

    pub fn set_value(&self, value: &Rc<usize>) {
        let count = Rc::strong_count(value);
        if count > self.max {
            self.messages.borrow_mut().push("Error: You can't go over your quota!".to_string());
        } else {
            self.value.set(count);
            let calculation = (count * 100) / self.max;
            if calculation >= 70 {
                let msg: String = format!("Warning: You have used up over {}% of your quota!", calculation);
                self.messages.borrow_mut().push(msg);
            }
        }
    }

}