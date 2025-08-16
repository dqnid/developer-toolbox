use arboard::Clipboard;
use colored::Colorize;
use inquire::Select;
use inquire::Text;
use std::env;

use crate::color::Color;
use crate::color::HSL;
use crate::color::HSV;
use crate::color::RGB;

mod color;
pub mod core;

fn set_clipboard(output: String) {
    let mut ctx = Clipboard::new().unwrap();
    let result = ctx.set_text(output.clone());
    match result {
        Ok(_) => {
            println!(
                "{} Copied {} to clipboard!",
                "✔ ".truecolor(0, 240, 0),
                output
            );
        }
        Err(_) => {
            println!("{} Error on copy to clipboard!", "✔ ".truecolor(240, 0, 0),);
        }
    }
}

// fn read_clipboard() -> Result<String, ()> {
// let ctx: ClipboardContext = ClipboardContext::new().unwrap();
// let value = ctx.get_rich_text();
// match value {
//     Ok(clipboard) => return Ok(clipboard),
//     Err(_) => return Err(()),
// }
// }

/**
 * Params
 * --clipboard / -c : read from clipboard first
 * --input <String> / -i <String> : dont read from clipboard nor input, simply try parsing the input
 */
fn main() {
    let args: Vec<String> = env::args().collect();

    let parsed_color = Color::try_parse("#F2FA01".to_string());
    match parsed_color {
        Ok(color) => {
            let options = list_color_options(color.clone());
            let rgb_color: RGB = color.clone().into();
            let rgb_tuple = rgb_color.clone().to_u8_tuple();
            let selected_format = Select::new(
                &format!(
                    "Encodings of color {}",
                    "⬤ ".truecolor(rgb_tuple.0, rgb_tuple.1, rgb_tuple.2)
                ),
                options,
            )
            .prompt();

            match selected_format {
                Ok(format) => {
                    set_clipboard(format.clone());
                }
                Err(_) => {
                    let _ = Text::new("Error on input read").prompt();
                }
            }
        }
        Err(()) => (),
    }
}

fn list_color_options(color: Color) -> Vec<String> {
    let mut options: Vec<String> = vec![];

    let rgb_color: RGB = color.clone().into();
    let hsl_color: HSL = color.clone().into();
    let hsv_color: HSV = color.clone().into();

    options.push(format!("{:X}", rgb_color));
    options.push(format!("{}", rgb_color));
    options.push(format!("{}", hsl_color));
    options.push(format!("{}", hsv_color));

    options
}
