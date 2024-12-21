#[allow(warnings)]
mod bindings;

use bindings::exports::component::fraction_with_wit::fraction::{Fraction, Guest};

struct Component;

impl Guest for Component {
    fn new(numerator: i32, denominator: i32) -> Fraction{
        Fraction { numerator, denominator }    
    }
}

bindings::export!(Component with_types_in bindings);
