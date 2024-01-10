use std::{fmt::Display, str::FromStr};

use regex::Regex;

struct PossibleNum {
    value: i32,
    char_start: i32,
    char_end: i32,
    line_height: i32,
}

struct Symbol {
    value: String,
    char_pos: i32,
    line_height: i32,
}

impl Display for PossibleNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Value: {:?}, from ({:?},{:?})",
            self.value, self.char_start, self.char_end
        )
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: ({}, {})",
            self.value, self.char_pos, self.line_height
        )
    }
}

fn part01(input: &str) -> i32 {
    let number_pattern = Regex::new(r"(\d+)").unwrap();
    let symbol_pattern = Regex::new(r"[^\d.]").unwrap();
    let mut numbers: Vec<PossibleNum> = vec![];
    let mut symbols: Vec<Symbol> = vec![];
    let lines = input.lines();
    for (i, line) in lines.enumerate() {
        let number_matches: Vec<PossibleNum> = number_pattern
            .find_iter(line)
            .map(|m| PossibleNum {
                value: m.as_str().parse().unwrap_or(0),
                char_start: m.start() as i32,
                char_end: m.end() as i32,
                line_height: i as i32,
            })
            .collect();
        let symbol_matches: Vec<Symbol> = symbol_pattern
            .find_iter(line)
            .map(|m| Symbol {
                value: String::from_str(m.as_str()).unwrap(),
                char_pos: m.end() as i32,
                line_height: i as i32,
            })
            .collect();
        numbers.extend(number_matches);
        symbols.extend(symbol_matches);
    }
    let mut valid_numbers: Vec<i32> = vec![];
    for num in numbers.iter() {
        for sym in symbols.iter() {
            if (num.line_height == sym.line_height - 1
                || num.line_height == sym.line_height + 1
                || sym.line_height == num.line_height)
                && (num.char_start)
            {
                println!("{sym}: {num}");
            }
        }
    }
    println!("{:?}", valid_numbers);
    1
}

fn main() {
    let example1 = include_str!("./01-example.txt");
    let a1 = part01(example1);
    assert_eq!(a1, 4361);
}
