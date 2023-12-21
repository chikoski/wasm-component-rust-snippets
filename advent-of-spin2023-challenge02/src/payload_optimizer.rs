use crate::family::Family;

#[derive(Debug)]
pub struct OptimizedPayload {
    pub weight: u32,
    pub kids: u32,
    pub capacity: u32,
}

impl OptimizedPayload {
    fn new(weight: u32, kids: u32, capacity: u32) -> OptimizedPayload {
        OptimizedPayload { weight, kids, capacity }
    }
    fn empty(capacity: u32) -> OptimizedPayload {
        OptimizedPayload::new(0, 0, capacity)
    }

    fn has_space_for(&self, family: &Family) -> bool {
        self.weight + family.weight <= self.capacity
    }

    fn add(&mut self, family: &Family) {
        self.weight += family.weight;
        self.kids += family.kids;
    }
}

pub fn optimize_payload(
    family_list: &mut [Family],
    capacity: u32,
) -> OptimizedPayload {
    family_list.sort();
    family_list.reverse();
    let mut payload = OptimizedPayload::empty(capacity);
    for family in family_list {
        if payload.has_space_for(family) {
            payload.add(family)
        }
    }
    payload
}