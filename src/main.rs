mod encoder;
mod solver;

use std::io::stdin;

fn main() {
    let mut test = String::new();
    loop {
        let mut line = String::new();
        match stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => test += &(line.clone()),
            _ => break,
        }
    }

    encoder::format_output(solver::solve_formula(encoder::get_tests(test[..test.len() - 1].to_string())));
    return;
}