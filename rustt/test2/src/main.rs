use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let text = &args[1];
    print_ascii_art_from_text(text.to_lowercase().as_str());
}

fn print_ascii_art_from_text(text: &str) {
    let mut vect = Vec::new();
    for c in text.chars() {
        for i in get_ascii_art_from_char(c).lines() {
            vect.push(i);
        }
    }
    println!("");
    for i in 0..5 {
        for s in 0..text.len() {
            let index = (s * 5) + i;
            if s == 0 {
                print!(" ");
            }
            print!("{} ", vect[index]);
        }
        println!("");
    }
    println!("");
}
fn get_ascii_art_from_char(c: char) -> &'static str {
    match c {
        'a' => "  █  \n █ █ \n█████\n█   █\n█   █\n",
        'b' => "████ \n█   █\n████ \n█   █\n████ \n",
        'c' => " ████\n█    \n█    \n█    \n ████\n",
        'd' => "████ \n█   █\n█   █\n█   █\n████ \n",
        'e' => "█████\n█    \n█████\n█    \n█████\n",
        'f' => "█████\n█    \n█████\n█    \n█    \n",
        'g' => " ████\n█    \n█  ██\n█   █\n ████\n",
        'h' => "█   █\n█   █\n█████\n█   █\n█   █\n",
        'i' => "█████\n  █  \n  █  \n  █  \n█████\n",
        'j' => "    █\n    █\n    █\n█   █\n ████\n",
        'k' => "█   █\n█  █ \n███  \n█  █ \n█   █\n",
        'l' => "█    \n█    \n█    \n█    \n█████\n",
        'm' => "█   █\n██ ██\n█ █ █\n█   █\n█   █\n",
        'n' => "█   █\n██  █\n█ █ █\n█  ██\n█   █\n",
        'o' => " ███ \n█   █\n█   █\n█   █\n ███ \n",
        'p' => "████ \n█   █\n████ \n█    \n█    \n",
        'q' => " ████\n█   █\n█   █\n█  ██\n ████\n",
        'r' => "████ \n█   █\n████ \n█  █ \n█   █\n",
        's' => " ████\n█    \n ████\n    █\n████ \n",
        't' => "█████\n  █  \n  █  \n  █  \n  █  \n",
        'u' => "█   █\n█   █\n█   █\n█   █\n ████\n",
        'v' => "█   █\n█   █\n█   █\n █ █ \n  █  \n",
        'w' => "█   █\n█   █\n█ █ █\n██ ██\n█   █\n",
        'x' => "█   █\n █ █ \n  █  \n █ █ \n█   █\n",
        'y' => "█   █\n █ █ \n  █  \n  █  \n  █  \n",
        'z' => "█████\n   █ \n  █  \n █   \n█████\n",
        '!' => "  █  \n  █  \n  █  \n     \n  █  \n",
        '?' => " ███ \n█   █\n   █ \n     \n  █  \n",
        ' ' => "     \n     \n     \n     \n     \n",
        '@' => " ███ \n█   █\n█ █ █\n█ █ █\n ███ \n",
        '$' => "  █  \n ████\n  █  \n████ \n  █  \n",
        '#' => " █ █ \n█████\n █ █ \n█████\n █ █ \n",
        '%' => "█   █\n   █ \n  █  \n █   \n█   █\n",
        '^' => "  █  \n █ █ \n█   █\n     \n     \n",
        '&' => " ███ \n█   █\n ███ \n█ █ █\n ███ \n",
        '*' => "     \n █ █ \n  █  \n █ █ \n     \n",
        '(' => "   █ \n  █  \n  █  \n  █  \n   █ \n",
        ')' => " █   \n  █  \n  █  \n  █  \n █   \n",
        '-' => "     \n     \n ███ \n     \n     \n",
        '_' => "     \n     \n     \n     \n█████\n",
        '+' => "     \n  █  \n ███ \n  █  \n     \n",
        '=' => "     \n ███ \n     \n ███ \n     \n",
        '[' => " ███ \n █   \n █   \n █   \n ███ \n",
        ']' => " ███ \n   █ \n   █ \n   █ \n ███ \n",
        '<' => "   █ \n  █  \n █   \n  █  \n   █ \n",
        '>' => " █   \n  █  \n   █ \n  █  \n █   \n",
        '1' => "  █  \n ██  \n  █  \n  █  \n ███ \n",
        '2' => " ███ \n█   █\n   █ \n  █  \n█████\n",
        '3' => " ███ \n█   █\n  ██ \n█   █\n ███ \n",
        '4' => "   █ \n  ██ \n █ █ \n█████\n   █ \n",
        '5' => "█████\n█    \n████ \n    █\n████ \n",
        '6' => " ███ \n█    \n████ \n█   █\n ███ \n",
        '7' => "█████\n    █\n   █ \n  █  \n █   \n",
        '8' => " ███ \n█   █\n ███ \n█   █\n ███ \n",
        '9' => " ███ \n█   █\n ████\n    █\n ███ \n",
        '0' => " ███ \n█   █\n█   █\n█   █\n ███ \n",
        '.' => "     \n     \n     \n  █  \n  █  \n",
        ',' => "     \n     \n     \n  █  \n █   \n",
        _ => "     \n     \n     \n     \n     \n",
    }
}
