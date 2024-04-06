mod bindings;

use std::cell::RefCell;

use bindings::exports::component::composition::message_handlable::Guest;
use bindings::exports::component::composition::message_handlable::GuestMessage;
use bindings::exports::component::composition::message_handlable::Message;

pub struct MessageImpl {
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

impl GuestMessage for MessageImpl {
    fn get(&self) -> i32 {
        self.value.borrow().value
    }

    fn post(&self, value: i32) {
        println!("Posted message: {}", value);
        self.value.borrow_mut().value = value;
    }
    fn get_message(value: i32) -> Message {
        let guest = MessageImpl::from(value);
        Message::new(guest)
    }
}

impl From<i32> for MessageImpl {
    fn from(value: i32) -> Self {
        let value = IntValueHolder::from(value);
        MessageImpl {
            value: RefCell::new(value),
        }
    }
}

struct Component;

impl Guest for Component {
    type Message = MessageImpl;
}

bindings::export!(Component with_types_in bindings);