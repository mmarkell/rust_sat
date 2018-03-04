mod input;
mod solver;

use std::vec::Vec;

fn main() {
    let mut tests_to_run: Vec<(String)> = Vec::new();
    tests_to_run.push(testify(test_1()));
    tests_to_run.push(testify(test_2()));
    tests_to_run.push(testify(test_3()));
    let tests: Vec<(input::FORMULA)> = input::get_tests(tests_to_run);
    solver::solve_formulas(tests);
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