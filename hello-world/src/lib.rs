cargo_component_bindings::generate!();

use std::io::BufWriter;

use bindings::exports::component::hello_world::dimension::{Guest as Dimension, Point};
use bindings::exports::component::hello_world::formatter::{FormatError, Guest as Formatter};
use ferris_says::say;

struct Component;

impl Formatter for Component {
    fn id(value: String) -> String {
        value
    }

    fn format(value: String) -> Result<String, FormatError> {
        let mut writer = BufWriter::new(Vec::new());
        if let Err(_) = say(&value, 70, &mut writer) {
            Err(FormatError::Unknown)
        } else {
            String::from_utf8(writer.buffer().to_vec()).map_err(|_| FormatError::Utf8)
        }
    }

    fn format_point(p: Point) -> Result<String, FormatError> {
        Ok(format!("({},{})", p.x, p.y))
    }
}

impl Dimension for Component {
    fn new_point(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}
