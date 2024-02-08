mod bindings;

use std::cell::RefCell;

use bindings::exports::component::composition::message_handlable::Guest;
use bindings::exports::component::composition::message_handlable::GuestMessage;
use wit_bindgen::Resource;

pub struct Message {
    value: RefCell<IntValueHolder>,
}

struct IntValueHolder {
    value: i32,
}

impl From<i32> for IntValueHolder {
    fn from(value: i32) -> Self {
        IntValueHolder { value }
    }
}

impl GuestMessage for Message {
    fn get(&self) -> i32 {
        self.value.borrow().value
    }

    fn post(&self, value: i32) {
        println!("Posted message: {}", value);
        self.value.borrow_mut().value = value;
    }
}

struct Component;

impl Guest for Component {
    fn get_message(value: i32) -> Resource<Message> {
        let value = IntValueHolder::from(value);
        let m = Message {
            value: RefCell::new(value),
        };
        Resource::new(m)
    }
}
