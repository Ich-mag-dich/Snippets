use std::env;
fn main() {
    let a = env::args();
    let config = Config::new(a).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    let line_num = config.line.parse::<u32>().unwrap();

    draw_triangle_center(line_num);
}
struct Config {
    line: String,
}

impl Config {
    fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let line = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a line string"),
        };
        Ok(Config { line })
    }
}

fn draw_triangle_center(num: u32) {
    let numbers = (1..num + 1).collect::<Vec<u32>>();
    numbers.iter().for_each(|x| {
        let mut line = String::new();
        for _ in 0..num - x {
            line.push(' ');
        }
        for _ in 0..x * 2 - 1 {
            line.push('*');
        }
        println!("{}", line);
    });
}
