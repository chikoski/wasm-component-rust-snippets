cargo_component_bindings::generate!();

use bindings::exports::example::grep::filter::{Guest, LineList};

struct Component;

impl Guest for Component {
    fn apply(input: LineList) -> Option<LineList> {
        Some(input)
    }
}
