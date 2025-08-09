use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

pub mod core;
mod transmuter;

#[cfg(test)]
#[path = "./test/transmuter.test.rs"]
mod test;

fn example() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    println!("{:?}", ctx.get_contents());
    ctx.set_contents("some string".to_owned()).unwrap();
}

fn main() {
    println!("Hello, world!");
    example();
}
