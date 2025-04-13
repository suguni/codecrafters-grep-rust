use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    let mut i = 0;

    let input_line = input_line.chars().collect::<Vec<_>>();
    let pattern = pattern.chars().collect::<Vec<_>>();

    while i < input_line.len() {
        if  match_next(&input_line, i, &pattern) {
            return true;
        }
        i += 1;
    }

    false
}

fn match_next(input_line: &[char], pos: usize, pattern: &[char]) -> bool {
    let mut pat_pos: usize = 0;
    let mut cur_pos = pos;
    let mut prev_char: Option<char> = if cur_pos == 0 { None } else { Some(input_line[cur_pos - 1]) };

    while cur_pos < input_line.len() && pat_pos < pattern.len() {
        let cur_char = input_line[cur_pos];
        let pat_char = pattern[pat_pos];

        if pat_char == '^' {
            if let Some(prev_char) = prev_char {
                if prev_char == '\n' {
                    pat_pos += 1;
                } else {
                    break;
                }
            } else {
                pat_pos += 1;
            }
        } else {
            if pat_char == '\\' && pattern[pat_pos + 1] == 'd' {
                if cur_char.is_numeric() {
                    pat_pos += 2;
                } else {
                    break;
                }
            } else if pat_char == '\\' && pattern[pat_pos + 1] == 'w' {
                if cur_char.is_alphanumeric() {
                    pat_pos += 2;
                } else {
                    break;
                }
            } else if let Some(group_end_pos) = is_char_groups(&pattern[pat_pos..]) {
                if pattern[pat_pos + 1] == '^' {
                    if pat_pos + 2 == group_end_pos {
                        pat_pos += group_end_pos + 1;
                    } else {
                        if !pattern[pat_pos + 2..pat_pos + group_end_pos].contains(&cur_char) {
                            pat_pos += group_end_pos + 1;
                        } else {
                            break;
                        }
                    }
                } else {
                    if pat_pos + 1 >= group_end_pos {
                        pat_pos += group_end_pos + 1;
                    } else {
                        if pattern[pat_pos + 1..pat_pos + group_end_pos].contains(&cur_char) {
                            pat_pos += group_end_pos + 1;
                        } else {
                            break;
                        }
                    }
                }
            } else {
                if pat_char == cur_char {
                    pat_pos += 1;
                } else {
                    break;
                }
            }

            prev_char = Some(cur_char);
            cur_pos += 1;
        }
    }

    pat_pos == pattern.len()
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
