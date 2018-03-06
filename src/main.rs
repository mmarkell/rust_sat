mod input;
mod solver;

use std::vec::Vec;
use std::io;
use std::io::BufRead;
use std::io::{Error, ErrorKind};
use std::io::stdin;

fn main() {
    let mut tests_to_run: Vec<(String)> = Vec::new();
    let mut test = String::new();
    loop {
        let mut line = String::new();
        match stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(x) => {
                if line == "\n" {
                    break;
                } else {
                    let l = line.clone();
                    test += &l;
                }
            },
            _ => break,
        }
    }
    tests_to_run.push(test[..test.len() -1].to_string());
    let tests: Vec<(input::FORMULA)> = input::get_tests(tests_to_run);
    solver::solve_formulas(tests);
    return;
}

fn testify<'a>(s: &'a str) -> String {
    return s.to_string();
}

fn test_1<'a>() -> (&'a str) {
    return 
    "-1
    1 2 3
    -1 4";
}

fn test_2<'a>() -> (&'a str) {
    return 
    "1";
}

fn test_3<'a>() -> (&'a str) {
    return
    "-1";
}