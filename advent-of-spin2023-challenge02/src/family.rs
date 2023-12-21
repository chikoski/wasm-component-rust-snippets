use std::cmp::Ordering;

#[derive(Eq, PartialEq, Ord, Debug)]
pub struct Family {
    pub kids: u32,
    pub weight: u32,
}

impl Family {
    pub fn new(kids: u32, weight: u32) -> Family {
        Family { kids, weight }
    }
    fn average_weight(&self) -> f32 {
        (self.weight as f32) / (self.kids as f32)
    }
}

impl PartialOrd<Self> for Family {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_average = self.average_weight();
        let other_average = other.average_weight();
        let bitmap = (
            self_average.total_cmp(&other_average),
            self.kids.cmp(&other.kids)
        );
        Some(match bitmap {
            (Ordering::Less, _) => Ordering::Greater,
            (Ordering::Equal, Ordering::Greater) => Ordering::Greater,
            (Ordering::Equal, Ordering::Equal) => Ordering::Equal,
            _ => Ordering::Less
        })
    }
}

#[cfg(test)]
mod test {
    use super::Family;

    #[test]
    fn test_order_with_same_kids() {
        let a = Family::new(5, 20);
        let b = Family::new(5, 28);
        assert_eq!(a > b, true);
    }

    #[test]
    fn test_order() {
        let a = Family::new(5, 20);
        let b = Family::new(6, 20);
        assert_eq!(b > a, true);
    }

    #[test]
    fn test_order_equal_average_weight() {
        let a = Family::new(5, 20);
        let b = Family::new(6, 24);
        assert_eq!(b > a, true);
    }

}
