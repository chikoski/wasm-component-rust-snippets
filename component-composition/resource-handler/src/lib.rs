mod bindings;

use bindings::exports::component::composition::message_handlable::Guest;
use bindings::exports::component::composition::message_handlable::GuestMessage;
use wit_bindgen::Resource;

pub struct Message<'a> {
    value: i32,
}

impl GuestMessage for Message {
    fn get(&self) -> i32 {
        self.value
    }

    fn post(&self, value: i32) {
        println!("Posted message: {}", value);
    }
}

struct Component;

impl Guest for Component {
    fn get_message(value: i32) -> Resource<Message> {
        let m = Message { value };
        Resource::new(m)
    }
}
