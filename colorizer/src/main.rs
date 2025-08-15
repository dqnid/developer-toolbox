use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

use crate::color::Color;
use crate::color::HSV;
use crate::color::RGB;

mod color;
pub mod core;

fn example() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    println!("{:?}", ctx.get_contents());
    ctx.set_contents("some string".to_owned()).unwrap();
}

fn main() {
    println!("Hello, world!");
    example();

    let color = RGB::new(220, 10, 50);
    let hsv_color = HSV::from(RGB::new(220, 10, 50));
    println!("RGB color: {}, HSV Color: {}", color, hsv_color);

    Color::try_parse("#F2FA01".to_string());
    Color::try_parse("rgb(1, 3,4)".to_string());
    Color::try_parse("rgb(1,3 ,4)".to_string());
    Color::try_parse("rgb(1,F,4)".to_string());
    Color::try_parse("hsl(100,100%,40%)".to_string());
    Color::try_parse("hsl(100,100%,40)".to_string());
}
