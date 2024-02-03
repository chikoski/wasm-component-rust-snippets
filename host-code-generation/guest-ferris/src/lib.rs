use std::io::BufWriter;

use bindings::exports::example::component::says::{FormatError, Guest};

mod bindings;

struct Component;

impl Guest for Component {
    fn format(message: String) -> Result<String, FormatError> {
        let mut writer = BufWriter::new(vec![]);
        ferris_says::say(&message, 70, &mut writer).map_err(|_| FormatError::Ioerror)?;
        let formatted = String::from_utf8(writer.buffer().to_vec()).map_err(|_| FormatError::ConvertError)?;
        Ok(formatted)
    }

    fn say(message: String) {
        let message = if let Ok(message) = Component::format(message) {
            message
        } else {
            "Something wrong happened".to_string()
        };
        println!("{}", message);
    }
}
