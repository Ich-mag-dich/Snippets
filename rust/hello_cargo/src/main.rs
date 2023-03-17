#[macro_use]
extern crate fstrings;
const PI: f32 = 3.141592;

fn add_func(x: i32, y: i32) -> i32 {
    return x + y;
}

fn main() {
    let mut name = "world?!";
    println_f!("hello, {name}");
    name = "world!";
    println_f!("hello, {name}");

    // as //

    let a = 13u8;
    let b = 7u32;
    let c = (a as u32) + b;
    println_f!("{c}");

    let t = false;
    println_f!("{}", t as u8);
    // println_f!("{t as u8}"); <- isn't working.

    // 상수 //

    println_f!("아무 재료 없이 애플 {PI}를 만들려면, 먼저 우주를 만들어야 한다.");

    // array //

    let nums: [i32; 3] = [1, 2, 3];
    println_f!("{nums:?}");
    println_f!("{}", nums[1]);
    // println_f!("{nums[0]}") <- isn't working.

    // function //

    println_f!("{}", add_func(42, 13));
    // println_f!("{add_func(42, 13)}"); <- isn't working.

    // pause //
    use std::process::Command;
    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}