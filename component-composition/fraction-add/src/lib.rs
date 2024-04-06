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
        let mut numerator = self.numerator.abs() as u32;
        let mut gcd = self.denominator;

        let mut r = numerator % gcd;
        while r != 0 {
            numerator = gcd;
            gcd = r;
            r = numerator % gcd;
        }
        return Fraction::new(self.numerator / (gcd as i32), self.denominator / gcd);
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

bindings::export!(Component with_types_in bindings);
