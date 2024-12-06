#[allow(warnings)]
mod bindings;

use std::cell::RefCell;

use bindings::exports::chikoski::wit_example::countable::{Guest, GuestCounter, Counter};

struct HostCounter {
    value: u32,
}

impl HostCounter {
    fn new() -> Self {
        let value = 0;
        HostCounter { value }
    }

    fn up(&mut self) {
        self.value += 1;
    }

    fn down(&mut self) {
        if self.value > 0 {
            self.value -= 1
        }
    }

    fn value_of(&self) -> u32 {
        self.value
    }
}

struct GuestCounterImpl {
    inner: RefCell<HostCounter>,
}

impl GuestCounterImpl {
    fn create() -> Self {
        let inner = HostCounter::new();
        let inner = RefCell::new(inner);
        GuestCounterImpl { inner }
    }
}

impl GuestCounter for GuestCounterImpl {
    fn new() -> Counter {
        let c = GuestCounterImpl::create();
        Counter::new(c)
    }

    fn up(&self) {
        self.inner.borrow_mut().up();
    }

    fn down(&self) {
        self.inner.borrow_mut().down();
    }

    fn value_of(&self) -> u32 {
        self.inner.borrow().value_of()
    }
}

struct Component;

impl Guest for Component {
    type Counter = GuestCounterImpl;
}

bindings::export!(Component with_types_in bindings);

#[cfg(test)]
mod test {
    mod host_counter {
        use crate::HostCounter;

        #[test]
        fn test_initial_value() {
            let c = HostCounter::new();
            assert_eq!(0, c.value);
        }

        #[test]
        fn test_up() {
            let mut c = HostCounter::new();
            c.up();
            assert_eq!(1, c.value);
        }

        #[test]
        fn test_down() {
            let mut c = HostCounter::new();
            c.up();
            c.down();
            assert_eq!(0, c.value);
        }

        #[test]
        fn test_counter_minimum_value() {
            let mut c = HostCounter::new();
            c.down();
            c.down();
            assert_eq!(0, c.value);
        }
    }
}

