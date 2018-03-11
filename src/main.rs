mod input;
mod solver;

use std::vec::Vec;
use std::io::stdin;

fn main() {
    let mut tests_to_run: Vec<(String)> = Vec::new();
    let mut test = String::new();
    loop {
        let mut line = String::new();
        match stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(1) => test += &(line.clone()),
            Ok(length) => {
                if line == "\n" {
                    break;
                } else {
                    test += &(line.clone());
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