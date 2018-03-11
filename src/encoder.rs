// extern crate tempfile;
use std::vec::Vec;
use std::collections::HashSet;

#[derive(Debug)]
pub struct FORMULA {
    pub clauses: HashSet<Vec<i32>>,
    pub units: Vec<i32>,
}

pub fn get_tests (test_descriptors: Vec<String>) -> Vec<(FORMULA)> {
    let mut tests = Vec::new();
    for s in test_descriptors {
        tests.push((test_s(s)));
    }
    return tests;
}

impl FORMULA {
    
    pub fn new (clauses: HashSet<Vec<i32>>, units: Vec<i32>) -> FORMULA {
        return FORMULA {
            clauses: clauses,
            units: units,
        }
    }

}

fn test_s(test_case: String) -> FORMULA {
    let mut clauses: HashSet<Vec<i32>> = HashSet::new();
    let mut units: Vec<i32> = Vec::new();

    for line in test_case.split("\n") {
        let mut v: Vec<i32> = Vec::new();
        for l in line.split(" ") {
            let r = l.parse::<i32>();
            if r.is_ok() {
                v.push(r.unwrap_or_default());
            }
        }
        let new_v = v.clone();
        let len = v.len();

        if len == 0 {
            clauses.insert(Vec::new());
        } else if len == 1 {
            units.push(new_v[0]);
        } else {
            clauses.insert(v);
        }
    }

    return FORMULA::new(clauses, units);
}

pub fn format_output(positives: Vec<i32>, negatives: Vec<i32>) {
    println!("{:?}", positives);
    println!("{:?}", negatives);
}