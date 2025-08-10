use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

pub mod core;
mod formats;
use formats::colors::{Color, RGB};

use crate::formats::colors::HSL;

fn example() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    println!("{:?}", ctx.get_contents());
    ctx.set_contents("some string".to_owned()).unwrap();
}

fn main() {
    println!("Hello, world!");
    example();

    let hsl_color = Color::from(HSL::new(193, 67, 28));
    let rgb_color = Color::from(RGB::from(HSL::new(193, 67, 28)));
    println!("HSL Color: {}", hsl_color.format());
    println!("RGB Color: {}", rgb_color.format());
}
