cargo_component_bindings::generate!();

use bindings::example::component::says::{format, say};

fn main() {
    let message = "Hello, world!";
    let error_message = "Something went wrong...".to_string();

    let formatted = format(message).unwrap_or(error_message);
    println!("Formatted by guest, printed by host:\n{}", formatted);

    let formatted = format!("Formatted by host, printed by guest:\n{}", message);
    say(&formatted);
}
