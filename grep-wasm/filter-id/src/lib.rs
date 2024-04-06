mod bindings;

use bindings::exports::example::grep::filter::{Guest, LineList};

struct Component;

impl Guest for Component {
    fn apply(input: LineList) -> Option<LineList> {
        Some(input)
    }
}

bindings::export!(Component with_types_in bindings);