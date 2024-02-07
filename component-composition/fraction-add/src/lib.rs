mod bindings;

use bindings::exports::component::composition::fraction_add::Fraction;
use bindings::exports::component::composition::fraction_add::Guest;

struct Component;

impl Fraction {
    fn new(numerator: i32, denominator: u32) -> Fraction {
        Fraction {
            numerator,
            denominator,
        }
    }
    fn add(&self, b: Fraction) -> Fraction {
        Self::new(
            self.numerator * (b.denominator as i32) + b.numerator * (self.denominator as i32),
            self.denominator * b.denominator,
        )
        .reduce()
    }
    fn sub(&self, b: Fraction) -> Fraction {
        Self::new(
            self.numerator * (b.denominator as i32) - b.numerator * (self.denominator as i32),
            self.denominator * b.denominator,
        )
        .reduce()
    }
    fn reduce(&self) -> Fraction {
        let mut numerator = self.numerator;
        let mut denominator = self.denominator as i32;
        while denominator != 0 {
            let gcd = denominator;
            denominator = numerator % denominator;
            numerator = gcd;
        }
        Fraction::new(numerator, denominator as u32)
    }
}

impl Guest for Component {
    fn new(numerator: i32, denominator: u32) -> Fraction {
        Fraction::new(numerator, denominator)
    }
    fn add(a: Fraction, b: Fraction) -> Fraction {
        a.add(b)
    }
    fn sub(a: Fraction, b: Fraction) -> Fraction {
        a.sub(b)
    }
    fn reduce(a: Fraction) -> Fraction {
        a.reduce()
    }
}
