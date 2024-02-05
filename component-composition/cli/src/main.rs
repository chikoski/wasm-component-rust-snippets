mod bindings;

use crate::bindings::component::composition::calculation::add;
use crate::bindings::component::composition::formattable::format as say;

fn main() {
    let a = 1;
    let b = 2;
    let original = format!("{a} + {b} = {}", add(a, b));
    match say(&original) {
        Ok(message) => println!("{message}"),
        Err(e) => println!("{e:?}"),
    }
}
