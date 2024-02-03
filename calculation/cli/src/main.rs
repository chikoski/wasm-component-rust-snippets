mod bindings;

use crate::bindings::component::calculation::calculation::add;

fn main() {
    let a = 1;
    let b = 2;
    println!("{a} + {b} = {}", add(a, b));
}
