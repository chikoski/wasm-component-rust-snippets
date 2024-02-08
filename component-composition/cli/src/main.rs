mod bindings;

use std::fmt::Display;

use crate::bindings::component::composition::calculation::add;
use crate::bindings::component::composition::formattable::format as say;
use crate::bindings::component::composition::fraction_add;
use crate::bindings::component::composition::message_handlable::{get_message, Message};

fn main() {
    calculation();
    fraction();
    signaling();
}

fn ferris_says(text: &str) {
    match say(text) {
        Ok(message) => println!("{message}"),
        Err(e) => println!("{e:?}"),
    }
}

fn calculation() {
    let a = 3;
    let b = 7;
    let sum = add(a, b);
    ferris_says(&format!("{} + {} = {}", a, b, sum));
}

fn fraction() {
    let denominator = add(3, 7);
    let a = fraction_add::new(1, denominator);
    let b = fraction_add::new(denominator as i32, denominator + 3);
    let sum = fraction_add::reduce(fraction_add::add(a, b));
    let difference = fraction_add::sub(a, b);
    ferris_says(&format!("{} + {} = {}", a, b, sum));
    ferris_says(&format!("{} - {} = {}", a, b, difference));
}

impl Display for fraction_add::Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

fn signaling() {
    let message = get_message(42);
    let value = message.get();
    ferris_says(&format!("Got message: {}", value));
    message.post(value);
}
