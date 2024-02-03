mod bindings;

use bindings::exports::example::grep::filter::{Guest, LineList};

struct Component;

impl Guest for Component {
    fn apply(input: LineList) -> Option<LineList> {
        Some(input)
    }
}
