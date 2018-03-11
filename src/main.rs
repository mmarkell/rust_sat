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
            Ok(_) => test += &(line.clone()),
            _ => break,
        }
    }

    /*
        I use a stack of tests so that I could test multiple tests rapidly
        by adding them to a stack and running them one after the other.
    */

    tests_to_run.push(test[..test.len() -1].to_string());
    let tests: Vec<(input::FORMULA)> = input::get_tests(tests_to_run);
    solver::solve_formulas(tests);
    return;
}