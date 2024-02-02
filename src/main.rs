use ferris_says::say;

// from the previous step
use std::io::{BufWriter, stdout};

fn hello_world() {
    let stdout = stdout();
    let message = "hello world";
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message, width, &mut writer).unwrap();
}

fn hello_world2() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "World, hello";
    let regions = [southern_germany, chinese, english];
    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn main() {
    hello_world();
    hello_world2();
}