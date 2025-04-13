use crate::CharCls::{AlphaNumeric, Digit, Literal, NegCharGroup, PosCharGroup};
use std::env;
use std::io;
use std::process;
use crate::Quantifier::*;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    let mut i = 0;

    let input_line = input_line.chars().collect::<Vec<_>>();
    let pattern = pattern.chars().collect::<Vec<_>>();

    while i < input_line.len() {
        if match_next(&input_line, i, &pattern) {
            return true;
        }
        i += 1;
    }

    false
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Quantifier {
    One,
    OneOrMore,
    ZeroOrMore,
    ZeroOrOne,
}

fn match_next(input_line: &[char], pos: usize, pattern: &[char]) -> bool {
    let mut pat_pos: usize = 0;
    let mut cur_pos = pos;
    let mut prev_char: Option<char> = if cur_pos == 0 {
        None
    } else {
        Some(input_line[cur_pos - 1])
    };

    while pat_pos < pattern.len() {
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
        } else if pat_char == '$' {
            if cur_pos >= input_line.len() || input_line[cur_pos] == '\n' {
                pat_pos += 1;
                cur_pos += 1;
            } else {
                break;
            }
        } else if cur_pos >= input_line.len() {
            let (_, quantifier, pattern_advance) = extract_pattern(&pattern[pat_pos..]);
            if quantifier == ZeroOrOne {
                pat_pos += pattern_advance;
            } else {
                break;
            }
        } else {
            let (char_cls, quantifier, pattern_advance) = extract_pattern(&pattern[pat_pos..]);
            let matched_count = match_char(&input_line[cur_pos..], &char_cls, quantifier);

            if matched_count > 0 {
                prev_char = Some(input_line[cur_pos + matched_count - 1]);
            } else {
                if quantifier != ZeroOrOne {
                    break;
                }
            }

            pat_pos += pattern_advance;
            cur_pos += matched_count;
        }
    }

    pat_pos == pattern.len()
}

fn extract_pattern(pattern: &[char]) -> (CharCls, Quantifier, usize) {
    let (char_cls, pattern_size) = extract_char_class(pattern);
    let (quantifier, quantifier_size) = extract_quantifier(&pattern[pattern_size..]);

    let mut pos = pattern_size + quantifier_size;

    if quantifier == OneOrMore {
        loop {
            let (next_char_cls, next_pattern_size) = extract_char_class(&pattern[pos..]);
            if next_char_cls != char_cls {
                break;
            }

            let (_, next_quantifier_size) = extract_quantifier(&pattern[pos + next_pattern_size..]);
            pos += next_pattern_size + next_quantifier_size;
        }
    }

    (char_cls, quantifier, pos)
}

#[derive(Debug, Eq, PartialEq)]
enum CharCls<'a> {
    Digit,
    AlphaNumeric,
    PosCharGroup(&'a [char]),
    NegCharGroup(&'a [char]),
    Literal(char),
}

fn extract_char_class(pattern: &[char]) -> (CharCls, usize) {
    if pattern[0] == '\\' && pattern[1] == 'd' {
        (Digit, 2)
    } else if pattern[0] == '\\' && pattern[1] == 'w' {
        (AlphaNumeric, 2)
    } else if pattern[0] == '[' {
        let is_negative = pattern[1] == '^';
        if let Some(end) = pattern.iter().position(|c| *c == ']') {
            if is_negative {
                (NegCharGroup(&pattern[2..end]), end + 1)
            } else {
                (PosCharGroup(&pattern[1..end]), end + 1)
            }
        } else {
            (Literal('['), 1)
        }
    } else if pattern[0] != '^' && pattern[0] != '$' {
        (Literal(pattern[0]), 1)
    } else {
        panic!("Unknown char pattern: {}", pattern[0]);
    }
}

fn extract_quantifier(pattern: &[char]) -> (Quantifier, usize) {
    if pattern.len() == 0 {
        (One, 0)
    } else if pattern[0] == '+' {
        (OneOrMore, 1)
    } else if pattern[0] == '*' {
        (ZeroOrMore, 1)
    } else if pattern[0] == '?' {
        (ZeroOrOne, 1)
    } else {
        (One, 0)
    }
}

fn match_char(input: &[char], char_cls: &CharCls, quantifier: Quantifier) -> usize {
    let mut pos = 0;
    let mut matched_count = 0;
    loop {
        let matched = match char_cls {
            Digit => input[pos].is_numeric() ,
            AlphaNumeric =>  input[pos].is_alphanumeric(),
            PosCharGroup(group) => group.contains(&input[pos]),
            NegCharGroup(group) => !group.contains(&input[pos]),
            Literal(c) => *c == input[pos],
        };

        if matched {
            matched_count += 1;
            if quantifier == One || pos == input.len() - 1 {
                break;
            }
        } else {
            break;
        }

        pos += 1;
    }
    matched_count
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


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_extract_char_class() {
        let pattern: Vec<char> = "[abcd]".chars().collect();
        let (cls, size) = extract_char_class(&pattern);
        assert_eq!(size, 6);
        assert_eq!(cls, PosCharGroup(&vec!['a', 'b', 'c', 'd']));
    }

    #[test]
    fn test_match_char() {
        let group: Vec<char> = "abcd".chars().collect();
        let matched_count = match_char(&vec!['a'], &PosCharGroup(&group), One);
        assert_eq!(matched_count, 1);
    }

    #[test]
    fn test_match_one_or_more_chars() {
        let matched_count = match_char(&vec!['a', 'a', 't'], &Literal('a'), OneOrMore);
        assert_eq!(matched_count, 2);
    }

    #[test]
    fn test_extract_char_cls_quantifier() {
        let pattern: Vec<char> = "ca+t".chars().collect();
        let (c1, q1, m1) = extract_pattern(&pattern[0..]);
        let (c2, q2, m2) = extract_pattern(&pattern[m1..]);
        let (c3, q3, m3) = extract_pattern(&pattern[m1+m2..]);

        assert_eq!(c1, Literal('c')); assert_eq!(q1, One); assert_eq!(m1, 1);
        assert_eq!(c2, Literal('a')); assert_eq!(q2, OneOrMore); assert_eq!(m2, 2);
        assert_eq!(c3, Literal('t')); assert_eq!(q3, One); assert_eq!(m3, 1);
    }

    #[test]
    fn test_literal_match() {
        assert!(match_pattern("tcat", "cat"));
        assert!(!match_pattern("ca", "cat"));
    }

    #[test]
    fn test_one_more_match_pattern() {
        assert!(match_pattern("caaat", "ca+t"));
        assert!(match_pattern("caaat", "ca+at"));
        assert!(!match_pattern("ca", "ca+t"));
    }

    #[test]
    fn test_zero_or_one_match_pattern() {
        assert!(match_pattern("dogs", "dogs?"));
        assert!(match_pattern("dog", "dogs?"));
    }
}
