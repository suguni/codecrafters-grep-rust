use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern.chars().count() == 1 {
        input_line.contains(pattern)
    } else if pattern.contains("\\d") {
        input_line.chars().find(|c| c.is_numeric()).is_some()
    } else if pattern.contains("\\w") {
        input_line.chars().find(|c| c.is_alphanumeric()).is_some()
    } else if is_positive_char_groups(pattern) {
        match_positive_char_groups(input_line, pattern)
    } else {
        panic!("Unhandled pattern: {}", pattern)
    }
}

fn is_positive_char_groups(pattern: &str) -> bool {
    pattern.starts_with('[') && pattern.ends_with(']')
}

fn match_positive_char_groups(input_line: &str, pattern: &str) -> bool {
    pattern[1..pattern.len() - 1]
        .chars()
        .find(|c| input_line.contains(*c))
        .is_some()
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // eprintln!("Logs from your program will appear here!");

    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    // Uncomment this block to pass the first stage
    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
