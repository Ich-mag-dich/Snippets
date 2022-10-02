#[macro_use]
extern crate fstrings;

fn main() {
    let mut name = "world?!";
    println_f!("hello, {name}");
    name = "world!";
    println_f!("hello, {name}");
}
