use std::io::BufWriter;

use bindings::exports::component::composition::formattable::{FormatError, Guest};

mod bindings;

struct Component;

impl Guest for Component {
    fn format(message: String) -> Result<String, FormatError> {
        let mut writer = BufWriter::new(vec![]);
        ferris_says::say(&message, 70, &mut writer).map_err(|_| FormatError::Ioerror)?;
        let formatted =
            String::from_utf8(writer.buffer().to_vec()).map_err(|_| FormatError::ConvertError)?;
        Ok(formatted)
    }
}
