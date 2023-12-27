cargo_component_bindings::generate!();

use bindings::exports::example::grep::file_as_multiline::{Guest, LineList};

struct Component;

impl Guest for Component {
    fn grep(input: LineList) -> Option<LineList> {
        Some(input)
    }
}
