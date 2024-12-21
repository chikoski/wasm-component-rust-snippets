mod fraction {
    struct Fraction {
        numerator: i32,
        denominator: i32,
    }

    impl Fraction {
        #[export_name = "fraction_new"]
        pub fn new(numerator: i32, denominator: i32) -> Self {
            Fraction {
                numerator,
                denominator,
            }
        }

        #[export_name = "fraction_add"]
        pub fn add(&self, other: &Fraction) -> Fraction {
            let numerator = self.numerator * other.denominator + other.numerator * self.denominator;
            let denominator = self.denominator * other.denominator;
            Fraction {
                numerator,
                denominator,
            }
        }
    }
}

#[no_mangle]
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
