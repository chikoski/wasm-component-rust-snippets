mod bindings;

use crate::bindings::exports::component::calculation::calculation::Guest;

struct Component;

impl Guest for Component {
    fn add(a: u32, b: u32) -> u32 {
        a + b
    }
}
