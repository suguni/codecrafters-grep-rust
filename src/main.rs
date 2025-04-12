use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    let mut i = 0;

    let input_line = input_line.chars().collect::<Vec<_>>();
    let pattern = pattern.chars().collect::<Vec<_>>();

    while i < input_line.len() {
        let consumed_pattern = match_next(&input_line[i..], &pattern);
        if consumed_pattern == pattern.len() {
            return true;
        }
        i += 1;
    }

    false
}

fn match_next(input_line: &[char], pattern: &[char]) -> usize {
    let mut ix: usize = 0;
    let mut px: usize = 0;

    while ix < input_line.len() && px < pattern.len() {
        if pattern[px] == '\\' && pattern[px + 1] == 'd' {
            if input_line[ix].is_numeric() {
                px += 2;
            } else {
                break;
            }
        } else if pattern[px] == '\\' && pattern[px + 1] == 'w' {
            if input_line[ix].is_alphanumeric() {
                px += 2;
            } else {
                break;
            } // [asdf] [^]
        } else if let Some(pos) = is_char_groups(&pattern[px..]) {
            if pattern[px + 1] == '^' {
                if px + 2 == pos {
                    px += pos + 1;
                } else {
                    if !pattern[px + 2..px + pos].contains(&input_line[ix]) {
                        px += pos + 1;
                    } else {
                        break;
                    }
                }
            } else {
                if px + 1 >= pos {
                    px += pos + 1;
                } else {
                    if pattern[px + 1..px + pos].contains(&input_line[ix]) {
                        px += pos + 1;
                    } else {
                        break;
                    }
                }
            }
        } else {
            if pattern[px] == input_line[ix] {
                px += 1;
            } else {
                break;
            }
        }

        ix += 1;
    }

    px
}

fn is_char_groups(pattern: &[char]) -> Option<usize> {
    if pattern[0] == '[' {
        pattern.iter().position(|c| *c == ']')
    } else {
        None
    }
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
